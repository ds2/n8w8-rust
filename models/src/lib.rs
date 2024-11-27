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

pub mod generated;

use crate::generated::n8w8::AuthBasicCredentials;
use async_trait::async_trait;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

/// by default, `ParameterData` is just another word for `HashMap`.
pub type ParameterData = HashMap<String, String>;

/// The contracts for a N8w8 test.
pub trait N8w8Test<P, T> {
    /// set parameters to the test.
    /// * `params` - some parameters to configure this test.
    fn set_params(&mut self, params: P);
    /// run the test.
    fn run_test(&mut self) -> BoxResult<()>;
    /// get the results from the test
    fn get_result(&self) -> N8w8TestResultValues<T>;
}

#[async_trait]
pub trait AsyncN8w8Test<P, T>
where
    P: Serialize + ?Sized,
    T: Serialize,
{
    /// run the test.
    async fn run_test(&mut self, params: &P) -> Result<N8w8TestResultValues<T>, N8w8TestErrors>;
}

#[derive(PartialEq, Eq, Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct N8w8TestResultValues<T> {
    pub start_time: u64,
    pub stop_time: u64,
    pub successful: bool,
    pub error_message: String,
    pub results: T,
}

pub const TEST_PARAM_NAME_URL: &str = "url";
pub const TEST_PARAM_NAME_CONNECTT0: &str = "connectT0";
pub const TEST_PARAM_NAME_READT0: &str = "readT0";
pub const TEST_PARAM_NAME_BASICAUTHUSERNAME: &str = "basicAuthUsername";
pub const TEST_PARAM_NAME_BASICAUTHPW: &str = "basicAuthPw";

//#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
#[must_use]
pub fn to_u32(i: u16) -> u32 {
    u32::from(i)
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct HttpTestResponse {
    pub url: String,
    pub duration: u64,
    pub response_code: u32,
    pub connection_error: bool,
}

impl Display for HttpTestResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(url={}, durr={}, respCode={}, connError={})",
            self.url, self.duration, self.response_code, self.connection_error
        )
    }
}

/// The parameters for a Http test.
#[derive(PartialEq, Clone, Default)]
pub struct HttpTestParams {
    /// the url under test
    pub url: ::std::string::String,
    /// some basic auth data if required
    pub basic_auth: AuthBasicCredentials,
    /// the connect timeout.
    pub connect_timeout: u32,
    /// the read timeout to wait for the server to respond.
    pub read_timeout: u32,
    /// the http method to use for testing.
    pub http_method: String,
    /// if required, the payload to send.
    pub http_payload: String,
}

/// Test errors.
#[derive(thiserror::Error, Debug)]
pub enum N8w8TestErrors {
    /// A simple test error with an error message.
    #[error("TestError: {0}")]
    TestError(String),
    #[error("Error when preparing the test: {0}")]
    PreparationError(String),
}
