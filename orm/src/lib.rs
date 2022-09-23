use std::time::Duration;

use sea_orm::*;

pub mod entities;
pub use entities::{prelude::*, *};

pub struct Query;

impl Query {
    pub fn get_all_states(db: &DbConn) {
        // here we would do something
    }
}

pub async fn get_db_connection(conn_string: String) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(conn_string.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    Database::connect(opt).await
}
