#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
use std::time::Duration;

use sea_orm::*;

pub mod entities;
use entities::state::Entity as State;
use entities::state::Model as StateModel;
pub use entities::{prelude::*, *};

pub struct Query;

impl Query {
    pub async fn get_all_states(db: &DbConn) -> Result<Vec<StateModel>, DbErr> {
        // here we would do something
        State::find().all(db).await
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
