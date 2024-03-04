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

use crate::errors::AgentErrors;
use nachtwacht_models::generated::n8w8::ProcMemInfo;
use std::fs::File;
use std::io::{BufReader, Read};
use tracing::debug;

/// Parses /proc/meminfo and returns its values.
#[cfg(target_os = "linux")]
pub async fn parse_proc_mem_info() -> Result<ProcMemInfo, AgentErrors> {
    debug!("Starting check for /proc/meminfo..");
    let mut str = String::new();
    let file = File::open("/proc/meminfo").expect("Error in reading /proc/meminfo");
    let mut buffer_reader = BufReader::new(file);
    buffer_reader
        .read_to_string(&mut str)
        .expect("Unable to read line");
    debug!("We got these lines: {}", str);
    let lines = str.lines();
    let mut mem_info = ProcMemInfo {
        MemTotal: 0,
        MemFree: 0,
        MemAvailable: 0,
        Buffers: 0,
        Cached: 0,
        SwapCached: 0,
        SwapTotal: 0,
        SwapFree: 0,
        special_fields: Default::default(),
    };
    for this_line in lines {
        debug!("Line to parse is: {}", this_line);
        let mut splits = this_line.split_whitespace();
        match splits.next().unwrap() {
            "MemTotal:" => {
                mem_info.MemTotal = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "MemFree:" => {
                mem_info.MemFree = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "MemAvailable:" => {
                mem_info.MemAvailable = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "Buffers:" => {
                mem_info.Buffers = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "Cached:" => mem_info.Cached = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024,
            "SwapCached:" => {
                mem_info.SwapCached = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "SwapTotal:" => {
                mem_info.SwapTotal = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            "SwapFree:" => {
                mem_info.SwapFree = splits.next().unwrap_or("0").parse().unwrap_or(0) * 1024
            }
            &_ => {}
        }
    }
    debug!("Returning this val: {}", mem_info);
    Ok(mem_info)
}

#[cfg(target_os = "macos")]
pub fn parse_proc_mem_info() -> Result<ProcMemInfo, AgentErrors> {
    Err(AgentErrors::NotImplemented())
}

#[cfg(test)]
mod tests {
    use crate::proc_meminfo::parse_proc_mem_info;
    use futures::executor::block_on;
    use std::sync::Once;
    use tracing::info;
    use tracing::level_filters::LevelFilter;

    static INIT: Once = Once::new();
    pub(crate) fn setup() {
        // some setup code, like creating required files/directories, starting
        // servers, etc.
        println!("Would setup servers here..");
        INIT.call_once(|| {
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
            info!("Logger should be enabled now!");
        });
    }

    #[cfg(target_os = "linux")]
    fn it_works() {
        setup();
        let result = block_on(parse_proc_mem_info());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.MemFree > 0);
        println!("Result: {:?}", result);
    }
}
