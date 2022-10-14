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
    log::info!("Init test..");
    let mut http_test = HttpCheckImpl::new();
    http_test.set_params(http_test_params);
    http_test.run_test(3).expect("Failed test");
    let test_result = http_test.get_result();
    if test_result.successful {
        //yeah
        log::info!("Hurra :)");
    } else {
        //not good
        log::warn!("Not good..")
    }
}
