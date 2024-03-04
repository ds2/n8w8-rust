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

use clap::{Parser, ValueEnum};
use tracing::debug;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

use nachtwacht_core::longhorn::get_node_by_id;

///This enum contains all the k8s operations we can perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum K8sOperations {
    ///Get information about a longhorn node.
    LonghornNodeInfo,
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// What operation to perform.
    #[clap(short, long)]
    operation: K8sOperations,
    ///The name of the kubernetes node to check.
    #[clap(short, long)]
    node: Option<String>,
    ///The url to the longhorn API. Usually something like http://localhost:11080/v1 or http://longhorn-ui.longhorn-system/v1
    #[clap(short, long)]
    longhorn_api_url: Option<Url>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    debug!("Checking for k8s stuff..");
    let args = CliArgs::parse();
    let operation: K8sOperations = args.operation;
    let lh_url_option: Option<Url> = args.longhorn_api_url;
    let node_option: Option<String> = args.node;
    match operation {
        K8sOperations::LonghornNodeInfo => {
            let lh_url = lh_url_option.expect("The longhorn_url is required!");
            let node = node_option.expect("The node name is required!");
            let lh_node_info = get_node_by_id(lh_url, node.as_str()).await?;
            let schedulable = match lh_node_info.allowScheduling {
                true => "schedulable",
                false => "not schedulable",
            };
            println!(
                "🖥️ {}, {}, with {} disk(s), tagged as {:?}:",
                lh_node_info.name,
                schedulable,
                lh_node_info.disks.len(),
                lh_node_info.tags
            );
            for (disk_name, lh_disk) in lh_node_info.disks {
                let disk_schedulable = match lh_disk.allowScheduling {
                    true => "schedulable",
                    false => "not schedulable",
                };
                println!(
                    "\t💾️ {} ({}), {}, mounted at {}, size {} = avail {} + resv {} + sched {}, tagged as {:?}",
                    disk_name,
                    lh_disk.diskUUID,
                    disk_schedulable,
                    lh_disk.path,
                    human_bytes(lh_disk.storageMaximum),
                    human_bytes(lh_disk.storageAvailable),
                    human_bytes(lh_disk.storageReserved),
                    human_bytes(lh_disk.storageScheduled),
                    lh_disk.tags
                );
                for (pvc_name, size1) in lh_disk.scheduledReplica {
                    println!("\t\t🔈 {}, size {}", pvc_name, human_bytes(size1));
                }
            }
        }
    }
    Ok(())
}

fn human_bytes(val: u64) -> String {
    let mut unit = "bytes";
    let mut calc_val: f64 = val as f64;
    if calc_val >= 1024.0 {
        unit = "kb";
        calc_val /= 1024.0;
    }
    if calc_val >= 1024.0 {
        unit = "Mb";
        calc_val /= 1024.0;
    }
    if calc_val >= 1024.0 {
        unit = "Gb";
        calc_val /= 1024.0;
    }
    format!("{:.3}{}", calc_val, unit)
}
