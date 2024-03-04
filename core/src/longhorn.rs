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

use protobuf_json_mapping::parse_from_str_with_options;
use tracing::{debug, error, warn};
use url::Url;

use nachtwacht_models::generated::longhorn::{ErrorBlock, LonghornNode, LonghornNodes};

use crate::errors::K8sErrors;

pub async fn get_nodes(lh_url: Url) -> Result<Vec<LonghornNode>, K8sErrors> {
    debug!("creating client with url={}", lh_url);
    let node_url = format!("{}/nodes", lh_url);
    let resp = reqwest::get(node_url)
        .await
        .map_err(|e| {
            error!("Error when downloading the node data: {}", e);
            K8sErrors::ReqwestNetworkError(e)
        })?
        .text()
        .await
        .map_err(|e| {
            error!("Error when translating the content into text: {}", e);
            K8sErrors::NotImplemented()
        })?;
    debug!("Got this response: {}", resp);
    let parse_options = protobuf_json_mapping::ParseOptions {
        ignore_unknown_fields: true,
        ..Default::default()
    };
    let res1 = parse_from_str_with_options::<LonghornNodes>(resp.as_str(), &parse_options)
        .map_err(|e| {
            error!("Could not parse given json! {}", e);
            K8sErrors::JsonParseError(e)
        })?;
    debug!("Got data collection: {}", res1);
    Ok(res1.data)
}

pub async fn get_node_by_id(lh_url: Url, id: &str) -> Result<LonghornNode, K8sErrors> {
    debug!("creating client with url={}", lh_url);
    let node_url = format!("{}/nodes/{}", lh_url, id);
    let resp = reqwest::get(node_url).await.map_err(|e| {
        error!("Error when downloading the node data: {}", e);
        K8sErrors::ReqwestNetworkError(e)
    })?;
    debug!("Got this response: {:?}", resp);
    let response_status = resp.status().as_u16();
    let content_body = resp.text().await.map_err(|e| {
        error!("Error when translating the content into text: {}", e);
        K8sErrors::NotImplemented()
    })?;
    debug!(
        "The text body received from LH looks like this: {}",
        content_body
    );
    let parse_options = protobuf_json_mapping::ParseOptions {
        ignore_unknown_fields: true,
        ..Default::default()
    };
    if response_status >= 400 {
        let res1 = parse_from_str_with_options::<ErrorBlock>(content_body.as_str(), &parse_options)
            .map_err(|e| {
                error!("Could not parse given json! {}", e);
                K8sErrors::JsonParseError(e)
            })?;
        warn!("Error when checking the LH API for a given node: {}", res1);
        return Err(K8sErrors::LhServerError(res1));
    }
    let res1 = parse_from_str_with_options::<LonghornNode>(content_body.as_str(), &parse_options)
        .map_err(|e| {
        error!("Could not parse given json! {}", e);
        K8sErrors::JsonParseError(e)
    })?;
    debug!("Got data collection: {}", res1);
    Ok(res1)
}
