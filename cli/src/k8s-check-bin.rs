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

mod emojis;
mod k8s_stuff;

use crate::k8s_stuff::{handle_node_count, handle_node_info};
use clap::Parser;
use clap::ValueEnum;
use k8s_openapi::api::core::v1::{Namespace, Node, Pod};
use kube::api::{ListParams, ObjectList};
use kube::config::{KubeConfigOptions, Kubeconfig};
use kube::{Api, Client, Config};
use kube_client::Error;
use tracing::debug;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum KubeInfoCmd {
    NodeCount,
    NodeInfo,
    ClusterVersion,
    Pods,
    Namespaces,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CmdArgs {
    /// the name of the cluster. If not set, the default context will be used.
    #[arg(short, long)]
    context: Option<String>,
    /// the namespace to use for querying namespaced objects
    #[arg(short, long, default_value = "default")]
    namespace: String,
    /// what we want to query
    #[arg(short, long, value_enum, default_value = "cluster-version")]
    info: KubeInfoCmd,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        // Use RUST_LOG environment variable to set the tracing level
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // Sets this to be the default, global collector for this application.
        .init();
    let cli = CmdArgs::parse();
    let kube_config = Kubeconfig::read().expect("Could not read kube config!");
    let this_context_name: String;
    if kube_config.current_context.is_some() {
        this_context_name = kube_config
            .current_context
            .as_ref()
            .expect("Could not get current context from kubeconfig file!")
            .to_string();
    } else {
        let ctx_opt: Option<String> = cli.context;
        this_context_name = ctx_opt.expect("Could not unpack the context from the args!");
    }
    debug!("Selected context is {}", this_context_name);
    let kco = KubeConfigOptions {
        context: Some(this_context_name),
        cluster: None,
        user: None,
    };
    let config: Config = Config::from_custom_kubeconfig(kube_config, &kco)
        .await
        .expect("Could not get config from kube config and context name!");
    let kube_config_client = Client::try_from(config).expect("Client config issue..");
    debug!("Client should be created now!");
    match cli.info {
        KubeInfoCmd::NodeCount => {
            handle_node_count(kube_config_client.clone()).await;
        }
        KubeInfoCmd::NodeInfo => {
            handle_node_info(kube_config_client.clone()).await;
        }
        KubeInfoCmd::ClusterVersion => {
            let api_version = kube_config_client
                .apiserver_version()
                .await
                .expect("Could not get api version!");
            debug!("Info about the cluster: {:?}", api_version);
            println!("Kubernetes: {}.{}", api_version.major, api_version.minor);
            println!("\tgit: {}", api_version.git_version);
        }
        KubeInfoCmd::Pods => {
            let all_pods =
                get_all_pods_from_namespace(kube_config_client.clone(), cli.namespace.as_str())
                    .await
                    .expect("Could not list all pods!");
            debug!("Pods in namespace {}:", cli.namespace.as_str());
            for pod in all_pods {
                println!("Pod: {}", pod.metadata.name.expect("Pod has no name?"));
                for container in pod.spec.expect("Could not load spec from pod!").containers {
                    println!("\tContainer {}", container.name);
                    println!(
                        "\t\tImage: {}",
                        container.image.expect("Could not find image??")
                    );
                    if container.resources.is_some() {
                        let resources = container
                            .resources
                            .expect("Could not load the resources for this container!");
                        println!(
                            "\t\tResourceLimits: Requests={:?}, Limits={:?}",
                            resources.requests.unwrap_or_default(),
                            resources.limits.unwrap_or_default()
                        );
                    }
                }
            }
        }
        KubeInfoCmd::Namespaces => {
            let all_namespaces = get_all_namespaces(kube_config_client.clone())
                .await
                .expect("Could not load all namespaces!");
            debug!("Namespaces in this cluster:");
            for namespace in all_namespaces {
                println!(
                    "{}",
                    namespace
                        .metadata
                        .name
                        .expect("Could not load namespace name!")
                );
            }
        }
    }
    Ok(())
}

async fn get_all_pods_from_namespace(
    client: Client,
    namespace: &str,
) -> Result<ObjectList<Pod>, Error> {
    let pod_api: Api<Pod> = Api::<Pod>::namespaced(client, namespace);
    let params = ListParams {
        label_selector: None,
        field_selector: None,
        timeout: None,
        bookmarks: false,
        limit: None,
        continue_token: None,
    };
    pod_api.list(&params).await
}

async fn get_all_namespaces(client: Client) -> Result<ObjectList<Namespace>, Error> {
    let pod_api: Api<Namespace> = Api::<Namespace>::all(client);
    let params = ListParams {
        label_selector: None,
        field_selector: None,
        timeout: None,
        bookmarks: false,
        limit: None,
        continue_token: None,
    };
    pod_api.list(&params).await
}
