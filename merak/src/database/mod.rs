use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

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

        db { conn }
    }
    pub async fn ping(&self) {
        self.conn.ping().await.unwrap()
    }
}
