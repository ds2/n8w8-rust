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

use async_trait::async_trait;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use nachtwacht_models::{AsyncN8w8Test, N8w8TestErrors, N8w8TestResultValues};
use reqwest::{header, StatusCode};
use tracing::debug;

pub struct CheckOciContainerExists {
    // params: CheckOciContainerParams,
}

impl Default for CheckOciContainerExists {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckOciContainerExists {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            // params: CheckOciContainerParams::default(),
        }
    }
    pub async fn get_tags(
        &self,
        params: &CheckOciContainerParams,
    ) -> Result<Vec<String>, N8w8TestErrors> {
        let url = format!("https://{}/v2/{}/tags/list", params.registry, params.image);
        debug!("Checking if image exists with url: {}", url);
        let client = create_client(params.clone().username, params.clone().password)?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| N8w8TestErrors::TestError(format!("Error sending request: {}", e)))?;
        if response.status().is_success() {
            let tags: serde_json::Value = response
                .json()
                .await
                .map_err(|e| N8w8TestErrors::TestError(format!("Error parsing response: {}", e)))?;
            Ok(tags["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default())
        } else {
            Err(N8w8TestErrors::TestError(format!(
                "Error getting all tags for image: {}",
                response.status()
            )))
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CheckOciContainerParams {
    pub registry: String,
    pub image: String,
    pub tag: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth_token: Option<String>,
}

impl CheckOciContainerParams {
    pub fn new(registry: String, repository: String, tag: String) -> Self {
        Self {
            registry,
            image: repository,
            tag,
            ..Self::default()
        }
    }
}

impl Default for CheckOciContainerParams {
    fn default() -> Self {
        Self {
            registry: "registry.example.com".to_string(),
            image: "image".to_string(),
            tag: "tag".to_string(),
            username: None,
            password: None,
            auth_token: None,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CheckOciContainerResult {}

#[async_trait]
impl AsyncN8w8Test<CheckOciContainerParams, CheckOciContainerResult> for CheckOciContainerExists {
    async fn run_test(
        &mut self,
        params: &CheckOciContainerParams,
    ) -> Result<N8w8TestResultValues<CheckOciContainerResult>, N8w8TestErrors> {
        let url = format!(
            "https://{}/v2/{}/manifests/{}",
            params.registry, params.image, params.tag
        );
        debug!("Checking if image exists with url: {}", url);
        let client = create_client(params.clone().username, params.clone().password)?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| N8w8TestErrors::TestError(format!("Failed to send request: {e}")))?;
        // print response headers
        for (key, value) in response.headers() {
            debug!(
                "Header received from request: {}={}",
                key,
                value.to_str().expect("No header value given to render")
            );
        }
        let response_status = response.status();
        let response_payload = response.text().await.map_err(|e| {
            N8w8TestErrors::TestError(format!("Failed to read response payload: {e}"))
        })?;
        debug!("Payload: {}", response_payload);
        // check if the response status is 200
        match response_status {
            StatusCode::OK => Ok(N8w8TestResultValues {
                start_time: 0,
                stop_time: 0,
                successful: true,
                error_message: String::new(),
                results: CheckOciContainerResult {},
            }),
            StatusCode::NOT_FOUND => Ok(N8w8TestResultValues {
                start_time: 0,
                stop_time: 0,
                successful: false,
                error_message: "image or tag not found!".to_string(),
                results: CheckOciContainerResult {},
            }),
            StatusCode::UNAUTHORIZED => Ok(N8w8TestResultValues {
                start_time: 0,
                stop_time: 0,
                successful: false,
                error_message: "you need to authenticate!".to_string(),
                results: CheckOciContainerResult {},
            }),
            StatusCode::FORBIDDEN => Ok(N8w8TestResultValues {
                start_time: 0,
                stop_time: 0,
                successful: false,
                error_message: "You are not allowed to use this http method!".to_string(),
                results: CheckOciContainerResult {},
            }),
            other => Ok(N8w8TestResultValues {
                start_time: 0,
                stop_time: 0,
                successful: false,
                error_message: format!("Unmapped status code: {other:?}"),
                results: CheckOciContainerResult {},
            }),
        }
    }
}

fn as_base64(p0: &str, p1: &str) -> String {
    BASE64_STANDARD.encode(format!("{p0}:{p1}"))
}

fn create_client(
    username: Option<String>,
    pw: Option<String>,
) -> Result<reqwest::Client, N8w8TestErrors> {
    let mut client_builder = reqwest::ClientBuilder::new();
    if username.is_some() {
        let username = username.unwrap();
        debug!("Using basic auth with user {}", username);
        let mut headers = header::HeaderMap::new();
        let b64val = as_base64(username.as_str(), pw.unwrap_or_default().as_str());
        debug!("Auth value should be: {}", b64val);
        // Consider marking security-sensitive headers with `set_sensitive`.
        let mut auth_value = header::HeaderValue::from_str(format!("Basic {}", b64val).as_str())
            .map_err(|e| N8w8TestErrors::PreparationError(e.to_string()))?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        client_builder = client_builder.default_headers(headers);
    }
    let client = client_builder
        .build()
        .map_err(|e| N8w8TestErrors::PreparationError(e.to_string()))?;
    Ok(client)
}
