use clap::{Parser, Subcommand};
use futures::executor::block_on;
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::apimachinery::pkg::version::Info;
use k8s_openapi::GetNodeV1APIResourcesResponse;
use kube::config::{Cluster, Context, KubeConfigOptions, Kubeconfig, NamedCluster, NamedContext};
use kube::{Client, Config};
use log::{debug, error, info, warn};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CmdArgs {
    /// the location of the kubeconfig file.
    #[arg(short, long, value_name = "FILE")]
    kube_config: PathBuf,
    /// the name of the cluster.
    #[arg(short, long)]
    context: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cli = CmdArgs::parse();
    let user_kube_config_pathbuf: PathBuf = cli.kube_config;
    let user_kube_config_path = user_kube_config_pathbuf.as_path();
    debug!("Kubeconfig to be {:?}", user_kube_config_pathbuf.as_path());
    if !user_kube_config_path.exists() {
        error!(
            "Given kubeconfig {:?} seems not be a file??",
            user_kube_config_pathbuf
        );
        exit(1);
    }
    let kube_config = Kubeconfig::read_from(user_kube_config_pathbuf.as_path())
        .expect("Could not read kube config!");
    let this_context_name: String;
    if kube_config.current_context.is_some() {
        this_context_name = kube_config
            .current_context
            .as_ref()
            .expect("Could not get current context from kubeconfig file!")
            .to_string();
    } else {
        let ctx_opt: Option<String> = cli.context;
        this_context_name = ctx_opt.expect("asdasd");
    }
    debug!("Selected context is {}", this_context_name);
    let kco = KubeConfigOptions {
        context: Some(this_context_name),
        cluster: None,
        user: None,
    };
    let config: Config = Config::from_custom_kubeconfig(kube_config, &kco)
        .await
        .expect("Could not get config from kubeconfig and context name!");
    let c3 = Client::try_from(config).expect("Client config issue..");
    let api_server_version: Info = c3
        .apiserver_version()
        .await
        .expect("Could not retrieve apiserver version!");
    info!("API Server: {:?}", api_server_version);
    Ok(())
}
