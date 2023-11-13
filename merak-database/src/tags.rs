use crate::Db;
use merak_entity::tag as TagModel;
use merak_entity::tag::Entity as Tag;

use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryOrder;
// use merak_entity::tag::Entity as TagModel;

impl Db {
    pub async fn get_all_tags(&self) {
        let tags = Tag::find().all(&self.conn).await;
        println!("tags: {:?}", tags)
    }

    pub async fn add_tag(&self) {
        let _ = TagModel::ActiveModel {
            name: Set("test".to_owned()),
            pid: Set(0.to_owned()),
            ..Default::default()
        }
        .save(&self.conn)
        .await;
    }

    pub async fn del_tag(&self, id: u32) {
        let _ = Tag::delete_by_id(id).exec(&self.conn).await;
    }

    pub async fn upd_tag(&self, id: u32) {}
}
