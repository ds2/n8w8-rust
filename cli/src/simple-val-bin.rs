// Copyright (C) 2023 Dirk Strauss
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

use crate::zabbix_mode::{get_zabbix_value, ZabbixValue};
use clap::Parser;
use futures::executor::block_on;

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
    let zabbix_val =
        block_on(get_zabbix_value(args.metric)).expect("Error when retrieving the zbx value!");
    println!("{}", zabbix_val);
}
