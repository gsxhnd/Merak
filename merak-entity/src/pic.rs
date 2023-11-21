use sea_orm::entity::prelude::*;
// use sea_orm::entity::;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
// use merak_entity::tag::Entity

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "mk_picture")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub fid: u32,
    pub created_at: DateTimeUtc,
    // pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
