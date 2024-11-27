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

use std::path::PathBuf;

use nachtwacht_models::generated::n8w8::{SwapNodeDetails, SwapProcessDetails};
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tracing::debug;

use crate::errors::AgentErrors;

pub async fn get_swap_usage(include_processes: bool) -> Result<SwapNodeDetails, AgentErrors> {
    Ok(SwapNodeDetails {
        processes: match include_processes {
            true => {
                let swap_processes = get_swap_processes(false, 0).await?;
                swap_processes
            }
            false => {
                vec![]
            }
        },
        special_fields: Default::default(),
    })
}

// for file in /proc/*/status ; do awk '/VmSwap|Name/{printf $2 " " $3}END{ print ""}' $file; done | sort -k 2 -n -r | less
#[cfg(target_os = "linux")]
pub async fn get_swap_processes(
    only_top_processes: bool,
    top_count: u16,
) -> Result<Vec<SwapProcessDetails>, AgentErrors> {
    let mut swap_processes: Vec<SwapProcessDetails> = Vec::new();
    let mut dir = fs::read_dir("/proc").await?;
    while let Some(child) = dir.next_entry().await? {
        // check if child exists
        if !child.path().exists() {
            debug!(
                "Child {} does not exist anymore! Skipping it.",
                child.path().display()
            );
            continue;
        }
        if !child.metadata().await?.is_dir() {
            continue;
        }
        let dir_name = child.file_name();
        let name_str = dir_name.as_os_str().to_str().unwrap_or_default();
        debug!("Directory is {}", name_str);
        let pid = name_str.parse::<u64>().unwrap_or_default();
        if pid == 0 {
            continue;
        }
        debug!("Needs check!");
        let status_file_path = format!("/proc/{}/status", pid);
        let status_file_p: PathBuf = PathBuf::from(status_file_path.clone());
        if status_file_p.exists() {
            debug!("Can read status file of this process!")
        } else {
            debug!(
                "Cannot read status file of this process {}! Will continue.",
                status_file_path
            );
            continue;
        }
        let file = File::open(status_file_p).await?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).await?;
        let mut swap_details = SwapProcessDetails::default();
        swap_details.processId = pid;
        for line in buffer.lines() {
            debug!("Read line of swap details: {}", line);
            if line.contains("VmSwap") {
                let swap_value = line.split_whitespace().nth(1).unwrap_or_default();
                let swap_value_int = swap_value.parse::<u64>().unwrap_or_default();
                swap_details.swapUsed = swap_value_int;
                if swap_details.swapUsed == 0 {
                    break;
                }
            } else if line.contains("Name") {
                let process_name = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or_default()
                    .to_string();
                swap_details.processName = process_name;
            } else if line.contains("PPid") {
                let parent_process_id = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or_default()
                    .parse::<u64>()
                    .unwrap_or_default();
                swap_details.parentProcessId = parent_process_id;
            }
        }
        if swap_details.swapUsed > 0 {
            debug!("Swap details: {}", swap_details);
            swap_processes.push(swap_details);
        }
        // let file = file.into_std().await;
    }
    // order by swapUsed
    swap_processes.sort_by(|a, b| b.swapUsed.cmp(&a.swapUsed));
    if only_top_processes {
        swap_processes.truncate(top_count as usize);
    }
    debug!("Swap processes count: {}", swap_processes.len());
    Ok(swap_processes)
}

#[cfg(test)]
mod swap_tests {
    use crate::swap::get_swap_processes;

    use std::sync::Once;
    use tracing::info;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::prelude::*;

    static INIT: Once = Once::new();
    pub(crate) fn setup_loggers() {
        // some setup code, like creating required files/directories, starting
        // servers, etc.
        INIT.call_once(|| {
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

    #[tokio::test]
    #[cfg(target_os = "linux")]
    pub async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        setup_loggers();
        let result = get_swap_processes(true, 10).await?;
        // assert!(result.len() > 1);
        println!("Result: {:?}", result);
        for swap_process in result {
            println!("* Swap process: {}", swap_process);
        }
        Ok(())
    }
}
