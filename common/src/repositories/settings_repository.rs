use crate::{db_conntext::DbContext, models::settings::Settings};

pub struct SettingsRepository {
    connection: DbContext,
}

impl SettingsRepository {
    pub fn new(connection: &DbContext) -> SettingsRepository {
        SettingsRepository {
            connection: connection.clone(),
        }
    }

    pub async fn get_highest_weight_settings(&self) -> Result<Settings, sqlx::Error> {
        #[cfg(debug_assertions)]
        let is_release = false;

        #[cfg(not(debug_assertions))]
        let is_release = true;

        sqlx::query_as::<_, Settings>(
            "
                SELECT *
                FROM settings
                WHERE is_release = ?
                ORDER BY weight
                LIMIT 1
                ",
        )
        .bind(is_release)
        .fetch_one(&self.connection.pool)
        .await
    }
}
