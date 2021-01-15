use crate::{db_connection::DbConnection, models::settings::Settings};

pub struct SettingsRepository {
    connection: DbConnection,
}

impl SettingsRepository {
    pub fn new(connection: &DbConnection) -> SettingsRepository {
        SettingsRepository {
            connection: connection.clone(),
        }
    }

    pub async fn get_highest_weight_settings(&self) -> Result<Settings, sqlx::Error> {
        sqlx::query_as::<_, Settings>(
            "
                SELECT *
                FROM settings
                ORDER BY weight
                LIMIT 1
                ",
        )
        .fetch_one(&self.connection.pool)
        .await
    }
}
