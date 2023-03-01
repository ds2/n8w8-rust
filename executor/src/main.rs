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

use log::{info, warn};
use nachtwacht_checks::http::HttpCheckImpl;
use nachtwacht_models::{HttpTestParams, N8w8Test};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::debug!("Nachtwacht Executor v{}", VERSION);
    let http_test_params = HttpTestParams {
        url: "https://pcwelt.de/".to_string(),
        basic_auth: Default::default(),
        connect_timeout: 0,
        read_timeout: 0,
        http_method: "GET".to_string(),
        http_payload: "".to_string(),
    };
    info!("Init test..");
    let mut http_test = HttpCheckImpl::new();
    http_test.set_params(http_test_params);
    http_test.run_test(3).expect("Failed test");
    let test_result = http_test.get_result();
    if test_result.successful {
        //yeah
        info!("Hurra :)");
    } else {
        //not good
        warn!("Not good..")
    }
}
