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

#[cfg(test)]
mod checks_unit_tests {
    use crate::oci::{CheckOciContainerExists, CheckOciContainerParams};
    use nachtwacht_models::{AsyncN8w8Test, N8w8Test};
    use tracing::info;

    use std::sync::Once;
    use tracing::level_filters::LevelFilter;

    static INIT: Once = Once::new();
    pub(crate) fn setup_loggers() {
        // some setup code, like creating required files/directories, starting
        // servers, etc.
        INIT.call_once(|| {
            use tracing_subscriber::prelude::*;
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
            info!("Logger should be enabled now!");
        });
    }

    #[tokio::test]
    async fn test_oci_container_exists() -> anyhow::Result<()> {
        setup_loggers();
        let mut check = CheckOciContainerExists::new();
        let params = CheckOciContainerParams {
            registry: "quay.io".to_string(),
            image: "ds2/enterpriselinux-rust".to_string(),
            tag: "almalinux9".to_string(),
            ..Default::default()
        };
        let result = check.run_test(&params).await?;
        info!("Result: {:?}", result);
        assert!(result.successful);
        Ok(())
    }
    #[tokio::test]
    async fn test_oci_container_not_exists() -> anyhow::Result<()> {
        setup_loggers();
        let mut check = CheckOciContainerExists::new();
        let params = CheckOciContainerParams::new(
            "quay.io".to_string(),
            "ds2/enterpriselinux-rust".to_string(),
            // there is no almalinux7
            "almalinux7".to_string(),
        );
        let result = check.run_test(&params).await?;
        info!("Result: {:?}", result);
        assert!(!result.successful);
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    async fn test_dockerio_postgres_17_exists() -> anyhow::Result<()> {
        setup_loggers();
        let mut check = CheckOciContainerExists::new();
        let params = CheckOciContainerParams::new(
            "docker.io".to_string(),
            "library/postgres".to_string(),
            "17".to_string(),
        );
        let result = check.run_test(&params).await?;
        info!("Result: {:?}", result);
        assert!(result.successful);
        Ok(())
    }
}
