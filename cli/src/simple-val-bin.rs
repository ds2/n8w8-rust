use crate::zabbix_mode::{get_zabbix_value, ZabbixValue};
use clap::Parser;

pub mod zabbix_mode;

/// Simple program to print single values to stdout.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which metric value should we print out.
    #[arg(short, long, value_enum)]
    metric: ZabbixValue,
}

fn main() {
    let args = Args::parse();
    let zabbix_val = get_zabbix_value(args.metric).expect("Error when retrieving the zbx value!");
    println!("{}", zabbix_val);
}
