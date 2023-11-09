use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePool;
use sqlx::Pool;
use sqlx::Sqlite;
use tokio::time::Sleep;

use super::DB;

#[derive(Clone)]
pub struct SqliteDatabase {
    pool: Pool<Sqlite>,
}

impl SqliteDatabase {
    pub async fn new() -> Self {
        let option = SqliteConnectOptions::new();
        let pool = SqlitePool::connect("sqlite://data/test.db").await.unwrap();
        pool.set_connect_options(option);

        SqliteDatabase { pool }
    }
}

impl DB for SqliteDatabase {
    fn test() {}
}

pub async fn new() -> Pool<Sqlite> {
    let option = SqliteConnectOptions::new();
    let pool = SqlitePool::connect("sqlite://data/test.db").await.unwrap();
    pool.set_connect_options(option);
    pool
}
