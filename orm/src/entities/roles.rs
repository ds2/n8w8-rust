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

//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub state: i64,
    pub created: DateTimeWithTimeZone,
    pub created_by: Uuid,
    pub modified: Option<DateTimeWithTimeZone>,
    pub modified_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::CreatedBy",
        to = "super::account::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Account2,
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::ModifiedBy",
        to = "super::account::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Account1,
    #[sea_orm(
        belongs_to = "super::state::Entity",
        from = "Column::State",
        to = "super::state::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    State,
    #[sea_orm(has_many = "super::roles_to_accounts::Entity")]
    RolesToAccounts,
}

impl Related<super::state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::State.def()
    }
}

impl Related<super::roles_to_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolesToAccounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
