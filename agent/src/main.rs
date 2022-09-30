use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::{thread, time};

use chrono::{DateTime, Utc};
use clap::{Parser, ValueEnum};
use daemonize::Daemonize;
use log::info;
use std::string::String;
use sysinfo::{DiskExt, System, SystemExt};

use nachtwacht_models::n8w8::{AgentDiscData, AgentNodeData};

use crate::errors::AgentErrors;
use crate::proc_loadavg::parse_proc_loadavg;
use crate::procstat::parse_proc_stat;
use crate::zabbix_mode::get_zabbix_value;

mod errors;
mod proc_loadavg;
mod procstat;
mod zabbix_mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ZabbixValue {
    Load1,
    Load5,
    Load15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RunMode {
    Agent,
    Zabbix,
}

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
    /// Which health value should we print out.
    #[arg(short, long, value_enum, default_value = "load1")]
    zabbix_value: ZabbixValue,
    /// defines the run mode for this agent. By default, we run in zabbix mode which means
    /// we only print out one value at the stdout.
    /// The other mode is the agent mode where the agent runs continuously in the background
    /// and sends the health data to a n8w8 endpoint.
    #[arg(short, long, value_enum, default_value = "zabbix")]
    mode: RunMode,
}

#[cfg(not(windows))]
fn main() {
    // Parse args first ;)
    let args = Args::parse();

    if args.mode == RunMode::Zabbix {
        let zabbix_val =
            get_zabbix_value(args.zabbix_value).expect("Error when retrieving the zbx value!");
        println!("{}", zabbix_val);
        exit(0);
    }
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
        // .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory(Path::new(args.workdir.as_str())) // for default behaviour.
        .user(args.user.as_str())
        // .group("adm") // Group name
        // .group(2) // or group id.
        // .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| println!("Should be running now. Please check via pid file! :)"))
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
                for disk in sys.disks() {
                    let this_disk_data = AgentDiscData {
                        device: disk.name().to_str().unwrap().to_string(),
                        mountpoint: disk.mount_point().to_str().unwrap().to_string(),
                        max_storage: disk.total_space(),
                        free_storage: disk.available_space(),
                        special_fields: Default::default(),
                    };
                    disk_vec.push(this_disk_data);
                }

                let cpu_proc_stats = parse_proc_stat().expect("TODO: panic message");
                info!("This machine has {} cores!", cpu_proc_stats.len());
                let agent_node_data = AgentNodeData {
                    hostname: sys.host_name().unwrap(),
                    load1: sys.load_average().one,
                    load5: sys.load_average().five,
                    load15: sys.load_average().fifteen,
                    totalMemory: sys.total_memory(),
                    usedMemory: sys.used_memory(),
                    freeMemory: sys.free_memory(),
                    totalSwap: sys.total_swap(),
                    usedSwap: sys.used_swap(),
                    freeSwap: sys.free_swap(),
                    kernelversion: sys.kernel_version().unwrap(),
                    cpudata: cpu_proc_stats,
                    disks: disk_vec,
                    os_name: sys.name().unwrap(),
                    os_version: sys.os_version().unwrap(),
                    special_fields: Default::default(),
                };
                println!("Node data is {}", agent_node_data);
                thread::sleep(sleep_time);
            }
        }
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
