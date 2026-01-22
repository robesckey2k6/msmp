use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "servers")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: String,
  pub sport: Option<i32>,
  pub dport: i32,
  pub version: Option<String>,
  pub status: Option<String>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
