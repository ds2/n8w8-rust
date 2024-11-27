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

use clap::Parser;
use nachtwacht_checks::oci::CheckOciContainerParams;
use nachtwacht_models::AsyncN8w8Test;
use tracing::debug;

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Tags {
        #[arg(short, long)]
        repository: String,
    },
    /// Checks for an OCI image.
    Check {
        /// The repository to check.
        #[arg(short, long)]
        repository: String,
        /// The tag to check for.
        #[arg(short, long)]
        tag: String,
    },
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CmdArgs {
    /// The registry to use.
    #[arg(long, env = "OCI_REGISTRY")]
    registry: String,
    /// The username to use for authentication
    #[arg(short, long, env = "OCI_USERNAME")]
    username: Option<String>,
    /// The password to use for authentication
    #[arg(short, long, env = "OCI_PASSWORD")]
    password: Option<String>,
    /// The auth token to use for authentication
    #[arg(short, long, env = "OCI_AUTH_TOKEN")]
    auth_token: Option<String>,
    /// What to do
    #[command(subcommand)]
    pub(crate) cmd: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = CmdArgs::parse();
    let registry = args.registry;
    let username = args.username;
    let password = args.password;
    let auth_token = args.auth_token;
    debug!("Registry: {:?}", registry);
    debug!("Username: {:?}", username);
    debug!("Password: {:?}", password);
    debug!("Auth Token: {:?}", auth_token);
    debug!("Command: {:?}", args.cmd);
    match args.cmd {
        Commands::Check { repository, tag } => {
            let params = CheckOciContainerParams {
                registry,
                image: repository,
                tag,
                username,
                password,
                auth_token,
            };
            let mut test = nachtwacht_checks::oci::CheckOciContainerExists::new();
            let result = test.run_test(&params).await?;
            debug!("Result: {:?}", result);
            match result.successful {
                true => {
                    println!("✅");
                    Ok(())
                }
                false => Err(anyhow::anyhow!("❌ {}", result.error_message)),
            }
        }
        Commands::Tags { repository } => {
            let params = CheckOciContainerParams {
                registry,
                image: repository,
                tag: "latest".to_string(),
                username,
                password,
                auth_token,
            };
            let test = nachtwacht_checks::oci::CheckOciContainerExists::new();
            let tags = test.get_tags(&params).await?;
            for tag in tags {
                println!("{}", tag);
            }
            Ok(())
        }
    }
}
