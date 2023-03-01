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

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

use std::time::Duration;

use sea_orm::*;

use entities::state::Entity as State;
use entities::state::Model as StateModel;
pub use entities::{prelude::*, *};

pub mod entities;

/// a dummy placeholder structure for some database queries attached to this structure.
pub struct Query;

///Some query functions.
impl Query {
    /// Should return all states from the database.
    pub async fn get_all_states(db: &DbConn) -> Result<Vec<StateModel>, DbErr> {
        // here we would do something
        State::find().all(db).await
    }
}

/// a method to return a database connection from the pool.
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
