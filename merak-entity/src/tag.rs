use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// impl Related<super::fruit::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Fruit.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
