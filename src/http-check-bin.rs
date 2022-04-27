
fn main(){
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Testing HTTP availability");
}
