pub mod n8w8;

use std::error::Error;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

use crate::n8w8::AuthBasicCredentials;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

pub type ParameterData = HashMap<String, String>;

pub trait N8w8Test<T> {
    fn set_params(&mut self, params: &ParameterData);
    fn run_test(&mut self, probe_count: u8) -> BoxResult<()>;
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

#[derive(PartialEq, Clone, Default)]
pub struct HttpTestParams {
    pub url: ::std::string::String,
    pub basic_auth: AuthBasicCredentials,
    pub connect_timeout: u32,
    pub read_timeout: u32,
    pub http_method: String,
    pub http_payload: String,
}
