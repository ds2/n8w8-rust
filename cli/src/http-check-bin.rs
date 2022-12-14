// Nachtwacht - A set of servers and client tools to monitor servers and services
// Copyright (C) 2022  Dirk Strauss
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::Parser;
use nachtwacht_checks::http::HttpCheckImpl;
use nachtwacht_models::n8w8::AuthBasicCredentials;
use nachtwacht_models::{HttpTestParams, HttpTestResponse, N8w8Test};
use std::process::exit;

pub mod zabbix_mode;

/*
das ist ein Commentary
 */

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The url to test
    #[clap(short, long)]
    url: String,
    /// the connect timeout, in ms
    #[clap(short = 'c', long = "connectt0", default_value_t = 5000)]
    connect_timeout: u32,
    /// the read timeout, in ms
    #[clap(short = 'r', long = "readt0", default_value_t = 10000)]
    read_timeout: u32,
    #[clap(short = 'b', long = "username", default_value = "")]
    username: String,
    #[clap(short = 'p', long = "password", default_value = "")]
    password: String,
}

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::debug!("Testing HTTP availability");
    let args = Cli::parse();

    let mut http_test = HttpCheckImpl {
        start_time: 0,
        end_time: 0,
        error_msg: "".to_string(),
        local_params: Default::default(),
        local_result: HttpTestResponse {
            url: "".to_string(),
            duration: 0,
            response_code: 0,
            connection_error: true,
        },
    };
    let mut basic_auth = Default::default();
    if args.username.len() > 0 {
        log::debug!(
            "Found username {}, will setup credential object..",
            args.username
        );
        basic_auth = AuthBasicCredentials {
            username: args.username,
            password: args.password,
            special_fields: Default::default(),
        };
    }
    http_test.set_params(HttpTestParams {
        url: args.url,
        basic_auth,
        connect_timeout: args.connect_timeout,
        read_timeout: args.read_timeout,
        http_method: "GET".to_string(),
        http_payload: "".to_string(),
    });
    let http_result = http_test.run_test(2);
    if http_result.is_err() {
        let http_error = http_result.err().unwrap();
        log::error!("A technical error occurred: {}", http_error);
        exit(1);
    }
    let test_result = http_test.get_result();
    if test_result.successful {
        log::info!(
            "test was successful, request took {}ms",
            test_result.results.duration
        )
    } else {
        log::error!("test was not successful! {}", test_result.error_message);
        exit(1);
    }
}
