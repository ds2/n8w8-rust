// Copyright (C) 2024 Dirk Strauss
//
// This file is part of Nachtwacht.
//
// Nachtwacht is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Nachtwacht is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io;

use actix_web::http::StatusCode;
use actix_web::{error, get, middleware, web, App, HttpServer, Responder};
use clap::Parser;
use lazy_static::lazy_static;
use log::{error, info};
use prometheus::{Gauge, HistogramOpts, HistogramVec, IntCounter, IntGauge, Registry};

use nachtwacht_core::proc_loadavg::parse_proc_loadavg;
use nachtwacht_core::proc_meminfo::parse_proc_mem_info;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref LOAD5: Gauge = Gauge::new("load_5", "The load value from the last 5 minutes")
        .expect("Error when setting up the gauge!");
    pub static ref FREE_MEMORY: IntGauge =
        IntGauge::new("free_memory", "The value of free memory, I hope in bytes")
            .expect("Error when setting up the gauge!");
    pub static ref INCOMING_REQUESTS: IntCounter =
        IntCounter::new("incoming_requests", "Incoming Requests").expect("metric can be created");
    pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec = HistogramVec::new(
        HistogramOpts::new("response_time", "Response Times"),
        &["env"]
    )
    .expect("metric can be created");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Port to bind to.
    #[arg(short, long, value_parser, default_value_t = 9105)]
    port: u16,
    /// The count of workers to use.
    #[arg(short, long, value_parser, default_value_t = 2)]
    workers: u8,
}

#[get("/metrics")]
async fn collect_metrics() -> impl Responder {
    INCOMING_REQUESTS.inc();
    let curr_memory_info = parse_proc_mem_info()
        .await
        .expect("Error when getting the procmem data!");
    let curr_load_info = parse_proc_loadavg()
        .await
        .expect("Error when getting load5!");
    LOAD5.set(curr_load_info.load5);
    FREE_MEMORY.set(curr_memory_info.MemFree as i64);
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        error!("could not encode custom metrics: {:?}", e);
    };
    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            error!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        error!("could not encode prometheus metrics: {:?}", e);
    };
    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            error!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_custom);
    format!("{}", res)
}

fn register_custom_metrics() {
    REGISTRY
        .register(Box::new(INCOMING_REQUESTS.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(RESPONSE_TIME_COLLECTOR.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(LOAD5.clone()))
        .expect("Error when registering the gauge!");
    REGISTRY
        .register(Box::new(FREE_MEMORY.clone()))
        .expect("Error when registering the gauge!");
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cli_args = CliArgs::parse();
    info!("Init..");
    register_custom_metrics();
    info!(
        "starting HTTP server at http://localhost:{}/metrics",
        cli_args.port
    );
    let workers: usize = cli_args.workers as usize;
    HttpServer::new(move || {
        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            // register favicon
            .service(collect_metrics)
            //.service(web::resource("/").to(index))
            .default_service(web::to(|| async {
                INCOMING_REQUESTS.inc();
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "Not implemented!"),
                    StatusCode::BAD_REQUEST,
                )
            }))
    })
    .bind(("0.0.0.0", cli_args.port))?
    .workers(workers)
    .run()
    .await
}
