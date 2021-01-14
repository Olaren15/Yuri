use crate::db_connection::DbConnection;
use crate::models::settings::Settings;
use sqlx::Row;

pub struct SettingsRepository {
    connection: DbConnection,
}

impl SettingsRepository {
    pub async fn new() -> SettingsRepository {
        SettingsRepository {
            connection: DbConnection::new().await,
        }
    }

    pub async fn get_highest_weight_settings(&self) -> Result<Settings, sqlx::Error> {
        let row = sqlx::query(
            "
                SELECT *
                FROM settings
                ORDER BY weight
                LIMIT 1
        ",
        )
        .fetch_one(&self.connection.pool)
        .await?;

        let settings = Settings {
            id: row.try_get("id")?,
            weight: row.try_get("weight")?,
            debug_token: row.try_get("debug_token")?,
            release_token: row.try_get("release_token")?,
            command_prefix: row.try_get("command_prefix")?,
        };

        Ok(settings)
    }
}
