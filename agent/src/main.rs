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

use std::fs::File;
use std::path::Path;
use std::string::String;
use std::{thread, time};

use chrono::{DateTime, Utc};
use clap::Parser;
use daemonize::Daemonize;
use futures::executor::block_on;
use sysinfo::{Disks, System};
use tracing::info;
use tracing::level_filters::LevelFilter;

use nachtwacht_core::proc_stat::parse_proc_stat;
use nachtwacht_models::generated::n8w8::{AgentDiscData, AgentNodeData};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the user who runs the daemon.
    #[arg(short, long, value_parser, default_value = "")]
    user: String,
    /// Location of the std output file to use
    #[arg(short, long, value_parser, default_value = "/tmp/n8w8-agent.log")]
    outfile: String,
    /// Location of the std err file to use
    #[arg(short, long, value_parser, default_value = "/tmp/n8w8-agent-error.log")]
    errfile: String,
    /// Location of the std output file to use
    #[arg(short, long, value_parser, default_value = "/tmp/n8w8-agent.pid")]
    pidfile: String,
    /// The work dir
    #[arg(short, long, value_parser, default_value = "/tmp")]
    workdir: String,
    /// Refresh timeout for the agent query loop.
    #[arg(short, long, value_parser, default_value_t = 5000)]
    refresh: u64,
}

#[cfg(target_os = "linux")]
fn main() {
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        // Use RUST_LOG environment variable to set the tracing level
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // Sets this to be the default, global collector for this application.
        .init();
    // Parse args first ;)
    let args = Args::parse();

    let stdout = File::create(args.outfile.as_str()).unwrap();
    let stderr = File::create(args.errfile.as_str()).unwrap();
    let sleep_time = time::Duration::from_millis(args.refresh);
    let p = Path::new(args.pidfile.as_str());

    //OK, we can start
    println!("Starting N8w8 Agent..");
    println!(
        "Will write logs to {}, errors to {}, and pid should be at {}, timeout of {}ms",
        args.outfile, args.errfile, args.pidfile, args.refresh
    );
    let daemonize = Daemonize::new()
        .pid_file(p) // Every method except `new` and `start`
        .working_directory(Path::new(args.workdir.as_str())) // for default behaviour.
        .user(args.user.as_str())
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| println!("Will enter loop now.."));

    match daemonize.start() {
        Ok(_) => {
            println!("Starting loop..");
            let mut sys = System::new_all();
            loop {
                let now: DateTime<Utc> = Utc::now();
                sys.refresh_all();
                let date = format!("UTC now is: {}", now);
                println!("Date is now {}", date);
                let mut disk_vec: Vec<AgentDiscData> = Vec::new();
                let mut disks = sysinfo::Disks::new();
                disks.refresh();
                for disk in &disks {
                    let this_disk_data = AgentDiscData {
                        device: disk.name().to_str().unwrap().to_string(),
                        mountpoint: disk.mount_point().to_str().unwrap().to_string(),
                        max_storage: disk.total_space(),
                        free_storage: disk.available_space(),
                        special_fields: Default::default(),
                    };
                    disk_vec.push(this_disk_data);
                }

                let cpu_proc_stats =
                    block_on(parse_proc_stat()).expect("Could not get /proc/stat details!");
                info!("This machine has {} cores!", cpu_proc_stats.len());
                let agent_node_data = AgentNodeData {
                    hostname: System::host_name().unwrap(),
                    load1: System::load_average().one,
                    load5: System::load_average().five,
                    load15: System::load_average().fifteen,
                    totalMemory: sys.total_memory(),
                    usedMemory: sys.used_memory(),
                    freeMemory: sys.free_memory(),
                    totalSwap: sys.total_swap(),
                    usedSwap: sys.used_swap(),
                    freeSwap: sys.free_swap(),
                    kernelversion: System::kernel_version().unwrap(),
                    cpudata: cpu_proc_stats,
                    disks: disk_vec,
                    os_name: System::name().unwrap(),
                    os_version: System::os_version().unwrap(),
                    special_fields: Default::default(),
                };
                println!("Node data is {}", agent_node_data);
                thread::sleep(sleep_time);
            }
        }
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
