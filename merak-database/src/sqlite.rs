use super::Db;

use merak_entity::tag as TagModel;
use sea_orm::ConnectionTrait;
use sea_query::{ColumnDef, Expr, Table};

impl Db {
    pub async fn init_sqlite(&self) {
        let stmt = Table::create()
            .if_not_exists()
            .table(TagModel::Entity)
            .col(
                ColumnDef::new(TagModel::Column::Id)
                    .unsigned()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(TagModel::Column::Name).not_null().string())
            .col(ColumnDef::new(TagModel::Column::Pid).not_null().unsigned())
            .col(
                ColumnDef::new(TagModel::Column::CreatedAt)
                    .date_time()
                    .null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();
        let builder = self.conn.get_database_backend();
        println!("{:?}", builder.build(&stmt));
        let _ = self.conn.execute(builder.build(&stmt)).await.unwrap();
    }
}
