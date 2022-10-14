use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

use crate::n8w8::AuthBasicCredentials;

pub mod n8w8;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

/// by default, ParameterData is just another word for HashMap.
pub type ParameterData = HashMap<String, String>;

/// The contracts for a N8w8 test.
pub trait N8w8Test<P, T> {
    /// set parameters to the test.
    /// * `params` - some parameters to configure this test.
    fn set_params(&mut self, params: P);
    /// run the test.
    ///
    /// * `probe_count` the maximum count of probes to take
    fn run_test(&mut self, probe_count: u8) -> BoxResult<()>;
    /// get the results from the test
    fn get_result(&self) -> N8w8TestResultValues<T>;
}

#[derive(PartialEq, Clone, Default)]
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
pub fn to_u32(i: u16) -> u32 {
    i as u32
}

#[derive(PartialEq, Clone, Default)]
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
