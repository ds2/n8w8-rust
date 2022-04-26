use std::error::Error;
use std::iter::Map;
use std::time::Instant;
use crate::models::n8w8::N8w8TestResultValues;
use crate::n8w8_capnp;

trait N8w8Test {
    fn set_params(params: Map<String, String>);
    fn run_test() -> dyn Error;
    fn get_result() -> N8w8TestResultValues;
}
