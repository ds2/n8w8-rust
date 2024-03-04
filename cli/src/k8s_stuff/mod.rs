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

use k8s_openapi::api::core::v1::Node;
use kube::api::{ListParams, ObjectList};
use kube_client::{Api, Client, Error};
use tracing::debug;

use crate::emojis::{COMPUTERS_EMOJI, DESKTOP_EMOJI};

pub async fn handle_node_count(client: Client) {
    let kube_nodes = get_all_nodes(client)
        .await
        .expect("Could not load all nodes!");
    println!("{}", kube_nodes.items.len());
}

pub async fn handle_node_info(kube_config_client: Client) {
    let kube_nodes = get_all_nodes(kube_config_client)
        .await
        .expect("Could not load all nodes!");
    debug!("Nodes of this cluster:");
    for kube_node in kube_nodes {
        let kube_node_status = kube_node.status.expect("Could not get the node status!");
        let kube_node_info = kube_node_status
            .clone()
            .node_info
            .expect("Could not get node info!");
        //let desktop_emoji = char::from_u32(0x1F5A5).expect("Not a valid code point");
        println!(
            "{} {}, ip={:?}, kube-version={}",
            DESKTOP_EMOJI,
            kube_node
                .metadata
                .name
                .expect("Could not load the node metadata!"),
            kube_node_status
                .clone()
                .addresses
                .expect("Could not find the node addresses!"),
            kube_node_info.kube_proxy_version
        );
        let node_capacities = kube_node_status
            .clone()
            .capacity
            .expect("Could not get the capacity");
        let cpus = node_capacities.get("cpu").expect("could not get CPU value");
        println!("\tCapacities: {:?}, cpu={:?}", node_capacities, cpus);
        debug!("This node info: {:?}", kube_node_info);
    }
}

async fn get_all_nodes(client: Client) -> Result<ObjectList<Node>, Error> {
    let pod_api: Api<Node> = Api::<Node>::all(client);
    let params = ListParams {
        label_selector: None,
        field_selector: None,
        timeout: None,
        bookmarks: false,
        limit: None,
        continue_token: None,
    };
    pod_api.list(&params).await
}
