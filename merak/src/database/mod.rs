use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_query::{ColumnDef, Table};
use std::time::Duration;

use merak_entity::tag as TagModel;

#[derive(Clone)]
pub struct db {
    conn: DatabaseConnection,
}

impl db {
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
        let stmt = Table::create()
            .table(TagModel::Entity)
            .if_not_exists()
            .col(
                ColumnDef::new(TagModel::Column::Id)
                    .unsigned()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .to_owned();
        let build = conn.get_database_backend();
        let _ = conn.execute(build.build(&stmt)).await;

        db { conn }
    }

    pub async fn ping(&self) {
        self.conn.ping().await.unwrap()
    }
}
