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

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::borrow::Borrow;
use std::thread;
use std::time::Duration;

use chrono::Local;
use reqwest::redirect::Policy;

use nachtwacht_models::generated::n8w8::AuthBasicCredentials;
use nachtwacht_models::{
    to_u32, BoxResult, HttpTestParams, HttpTestResponse, N8w8Test, N8w8TestResultValues,
    ParameterData, TEST_PARAM_NAME_BASICAUTHPW, TEST_PARAM_NAME_BASICAUTHUSERNAME,
    TEST_PARAM_NAME_CONNECTT0, TEST_PARAM_NAME_READT0, TEST_PARAM_NAME_URL,
};

/// The structure for a simple Http check.
pub struct HttpCheckImpl {
    /// the utc timestamp when the test started
    pub start_time: u64,
    /// the utc timestamp when the test finished
    pub end_time: u64,
    /// a possible error message
    pub error_msg: String,
    /// the test parameters
    pub local_params: HttpTestParams,
    /// the test response
    pub local_result: HttpTestResponse,
}

/// Implementations for the http check structure.
impl HttpCheckImpl {
    /// instantiates the http check structure.
    pub fn new() -> HttpCheckImpl {
        HttpCheckImpl {
            start_time: 0,
            end_time: 0,
            error_msg: "".to_string(),
            local_params: Default::default(),
            local_result: Default::default(),
        }
    }
    pub fn get_params(&self) -> ParameterData {
        let rc = ParameterData::new();

        rc
    }
}

/// the implementations for a N8w8Test using HttpCheckImpl.
impl N8w8Test<HttpTestParams, HttpTestResponse> for HttpCheckImpl {
    fn set_params(&mut self, params: HttpTestParams) {
        self.local_params = params;
    }

    fn run_test(&mut self, probe_count: u8) -> BoxResult<()> {
        self.start_time = Local::now().timestamp_millis() as u64;
        let url = url::Url::parse(&self.local_params.url);
        if url.is_ok() {
            let this_url = url.unwrap();
            let http_method = "GET";
            let max_timeout_value = self.local_params.connect_timeout;
            log::info!(
                "Checking url {} with connect timeout={:?}ms",
                this_url,
                max_timeout_value
            );
            let mut test_result = test_url(
                &this_url,
                max_timeout_value as u64,
                http_method.borrow(),
                &self.local_params.basic_auth,
            );
            for _ in 1..probe_count {
                if test_result.not_successful() {
                    log::debug!("test before was unsuccessful, try retest..");
                    thread::sleep(Duration::from_secs(5));
                    test_result = test_url(
                        &this_url,
                        max_timeout_value as u64,
                        http_method.borrow(),
                        self.local_params.basic_auth.borrow(),
                    );
                } else {
                    break;
                }
            }
            log::debug!("done with check thread");
            if test_result.not_successful() {
                self.error_msg = format!(
                    "Error http={} for {}",
                    test_result.response_code, test_result.url
                );
            }
            self.local_result = test_result;
        } else {
            let url_error = url.err().unwrap();
            self.error_msg = format!(
                "Error when parsing url {}: {}",
                self.local_params.url,
                url_error.to_string()
            );
            log::warn!("{}", self.error_msg);
        }
        self.end_time = Local::now().timestamp_millis() as u64;
        Ok(())
    }

    fn get_result(&self) -> N8w8TestResultValues<HttpTestResponse> {
        let rc = N8w8TestResultValues::<HttpTestResponse> {
            start_time: self.start_time,
            stop_time: self.end_time,
            successful: !self.local_result.not_successful(),
            error_message: self.error_msg.to_string(),
            results: HttpTestResponse {
                url: self.local_result.url.to_string(),
                duration: self.local_result.duration,
                response_code: self.local_result.response_code,
                connection_error: self.local_result.connection_error,
            },
        };
        return rc;
    }
}

pub trait HttpTestResponseTrait {
    fn not_successful(&self) -> bool;
}

impl HttpTestResponseTrait for HttpTestResponse {
    fn not_successful(&self) -> bool {
        return self.connection_error || self.response_code >= 400;
    }
}

pub fn test_url(
    url: &url::Url,
    t0: u64,
    http_method: &str,
    basic_auth: &AuthBasicCredentials,
) -> HttpTestResponse {
    log::debug!("creating client with url={}, t0={}..", url, t0);

    let mut client_builder = reqwest::blocking::Client::builder().redirect(Policy::limited(20));
    if t0 > 0 {
        log::debug!("Setting timeout to {}", t0);
        //this is all: read, connect etc.
        client_builder = client_builder.timeout(Some(Duration::from_secs(t0)));
        //this here is just connect
        client_builder = client_builder.connect_timeout(Some(Duration::from_secs(t0)));
    }
    let client = client_builder.build().unwrap();
    let mut auth_string: String = "".to_string();
    if basic_auth.username.len() > 0 {
        log::debug!(
            "Username found, will convert to base64 the user {}",
            basic_auth.username
        );
        let basic_auth_value =
            to_base64(basic_auth.username.as_str(), basic_auth.password.as_str());
        let fmt1 = format!("Basic {}", basic_auth_value);
        auth_string = fmt1;
    }
    log::debug!("note start time ..");
    let start_time = Local::now();
    log::debug!("Performing GET request with client..");
    let res;
    match http_method {
        "GET" => {
            let mut req_b = client.get(url.to_string());
            if auth_string.len() > 0 {
                log::debug!("Setting auth header {}", auth_string);
                req_b = req_b.header("Authorization", auth_string);
            }
            res = req_b.send();
        }
        _ => {
            todo!("This HTTP method is not yet supported!")
        }
    }
    log::debug!("OK, having a result, parsing it..");
    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time).to_std().unwrap();
    log::debug!("Duration was {}", duration.as_millis());
    let mut response_object = HttpTestResponse {
        url: url.to_string(),
        duration: duration.as_millis() as u64,
        response_code: 0,
        connection_error: false,
    };
    if res.is_ok() {
        let http_status_code = res.unwrap().status();
        log::debug!("Status for {}: {}", url, http_status_code);
        response_object.response_code = to_u32(http_status_code.as_u16());
    } else {
        log::warn!(
            "- technical error when connecting to url: {:?}",
            res.err().unwrap()
        );
        response_object.connection_error = true
    }
    log::debug!(
        "Sending back response object: {}",
        response_object.to_string()
    );
    response_object
}

fn to_base64(p0: &str, p1: &str) -> String {
    let token = format!("{}:{}", p0, p1);
    STANDARD.encode(token)
}

pub fn create_parameter_map_from_params(params: HttpTestParams) -> ParameterData {
    return create_parameter_map(
        params.url,
        params.connect_timeout,
        params.read_timeout,
        params.basic_auth,
    );
}

pub fn create_parameter_map(
    url: String,
    connect_t0: u32,
    read_t0: u32,
    basic_auth: AuthBasicCredentials,
) -> ParameterData {
    let mut rc: ParameterData = Default::default();
    rc.insert(TEST_PARAM_NAME_URL.to_string(), url);
    if connect_t0 > 0 {
        rc.insert(
            TEST_PARAM_NAME_CONNECTT0.to_string(),
            connect_t0.to_string(),
        );
    }
    if read_t0 > 0 {
        rc.insert(TEST_PARAM_NAME_READT0.to_string(), read_t0.to_string());
    }
    if basic_auth.username.len() > 0 {
        rc.insert(
            TEST_PARAM_NAME_BASICAUTHUSERNAME.to_string(),
            basic_auth.username,
        );
    }
    if basic_auth.password.len() > 0 {
        rc.insert(TEST_PARAM_NAME_BASICAUTHPW.to_string(), basic_auth.password);
    }
    return rc;
}
