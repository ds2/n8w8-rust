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

use crate::errors::AgentErrors;
use log::debug;
use nachtwacht_models::n8w8::ProcLoadavg;
use std::fs::File;
use std::io::{BufReader, Read};

#[cfg(target_os = "linux")]
pub fn parse_proc_loadavg() -> Result<ProcLoadavg, AgentErrors> {
    debug!("Trying to get loadavg..");
    let mut str = String::new();
    let file = File::open("/proc/loadavg").expect("Error in reading /proc/loadavg");
    let mut buffer_reader = BufReader::new(file);
    buffer_reader
        .read_to_string(&mut str)
        .expect("Unable to read line");
    debug!("The string value to parse is: {}", str);
    let mut splits = str.split_whitespace();
    let load1: f64 = splits.next().unwrap_or("0.0").parse().unwrap();
    let load5: f64 = splits.next().unwrap_or("0.0").parse().unwrap();
    let load15: f64 = splits.next().unwrap_or("0.0").parse().unwrap();
    let process_count_str = splits.next().unwrap_or("0/0");
    let mut splits2 = process_count_str.split("/");
    let running_processes: u32 = splits2.next().unwrap().parse().unwrap_or(0);
    let total_processes: u64 = splits2.next().unwrap().parse().unwrap_or(0);
    let latest_process_id: u64 = splits.next().unwrap_or("0").parse().unwrap_or(0);
    let load_avg = ProcLoadavg {
        load1,
        load5,
        load15,
        runningProcesses: running_processes,
        totalProcesses: total_processes,
        latestProcessId: latest_process_id,
        special_fields: Default::default(),
    };
    Ok(load_avg)
}

#[cfg(target_os = "macos")]
pub fn parse_proc_loadavg() -> Result<ProcLoadavg, AgentErrors> {
    Err(AgentErrors::NotImplemented())
}

#[cfg(test)]
mod tests {
    use crate::proc_loadavg::parse_proc_loadavg;

    #[test]
    #[cfg(target_os = "linux")]
    fn it_works() {
        let result = parse_proc_loadavg();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.load1 >= 0.0);
        println!("Result: {:?}", result);
    }
}
