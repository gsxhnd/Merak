pub mod tag;
use merak_entity::tag as TagModel;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_query::{ColumnDef, Expr, Table};
use std::time::Duration;

#[derive(Clone)]
pub struct Db {
    pub conn: DatabaseConnection,
}

impl Db {
    pub async fn new() -> Self {
        let mut opt = ConnectOptions::new("sqlite://data/test.db?mode=rwc");
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(true);
        let conn = Database::connect(opt)
            .await
            .expect("Database connection failed");

        Db { conn }
    }

    pub async fn init(&self) {
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
            .col(
                ColumnDef::new(TagModel::Column::UpdatedAt)
                    .date_time()
                    .null(), // .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP"),
            )
            .to_owned();
        let builder = self.conn.get_database_backend();
        println!("{:?}", builder.build(&stmt));
        let _ = self.conn.execute(builder.build(&stmt)).await.unwrap();
    }

    pub async fn ping(&self) {
        self.conn.ping().await.unwrap()
    }
}
