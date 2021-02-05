use std::env;

use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct DbContext {
    pub pool: MySqlPool,
}

impl DbContext {
    pub async fn new() -> DbContext {
        let conn_string =
            env::var("yuri_db").expect("Failed to retrieve connection string from environment");

        DbContext {
            pool: MySqlPool::connect(conn_string.as_str())
                .await
                .expect("Failed to connect to database"),
        }
    }
}
