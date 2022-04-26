extern crate capnp_rpc;

use std::borrow::Borrow;
use capnp::capability::Promise;
use capnp::Error;
use crate::n8w8_capnp;
use crate::n8w8_capnp::{http_check_params};
use crate::n8w8_capnp::n8w8_test;
use crate::n8w8_capnp::n8w8_test::{GetResultParams, GetResultResults, RunTestParams, RunTestResults, SetParamsParams, SetParamsResults};

pub struct HttpCheckImpl {
    pub params :http_check_params::Owned,
}

impl n8w8_test::Server for HttpCheckImpl {

    fn set_params(&mut self, _: SetParamsParams, _: SetParamsResults) -> Promise<(), Error> {
        Promise::ok(())
    }
    fn run_test(&mut self, _: RunTestParams, _: RunTestResults) -> Promise<(), Error> {
        println!("received a request for HttpCheck!");
        Promise::ok(())
    }
    fn get_result(&mut self, _: GetResultParams, _: GetResultResults) -> Promise<(), Error> {
        ::capnp::capability::Promise::err(
            ::capnp::Error::unimplemented(
                "method not implemented".to_string()
            )
        )
    }
}
