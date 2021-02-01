use std::env;

use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct DbConnection {
    pub pool: MySqlPool,
}

impl DbConnection {
    pub async fn new() -> DbConnection {
        let conn_string =
            env::var("yuri_db").expect("Failed to retrieve connection string from environment");

        DbConnection {
            pool: MySqlPool::connect(conn_string.as_str())
                .await
                .expect("Failed to connect to database"),
        }
    }
}
