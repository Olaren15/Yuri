use sqlx::Row;

use crate::db_conntext::DbContext;

pub struct GuildRepository {
    connection: DbContext,
}

impl GuildRepository {
    pub fn new(conn: &DbContext) -> GuildRepository {
        GuildRepository {
            connection: conn.clone(),
        }
    }

    pub async fn register_guild(&self, guild_id: u64) -> Result<(), String> {
        if let Ok(rows) = sqlx::query(
            "
                    SELECT id
                    FROM guilds
                    WHERE id = ?
                ",
        )
        .bind(guild_id)
        .fetch_all(&self.connection.pool)
        .await
        {
            if rows.is_empty() || (rows.first().is_some() && rows.first().unwrap().is_empty()) {
                match sqlx::query(
                    "
                            INSERT INTO guilds(id)
                            VALUES(?)
                        ",
                )
                .bind(guild_id)
                .execute(&self.connection.pool)
                .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.as_database_error().unwrap().to_string()),
                }
            } else {
                Err(String::from("This server is already registered."))
            }
        } else {
            Err(String::from(
                "Error registering this server, please try again.",
            ))
        }
    }

    pub async fn get_registered_guild_ids(&self) -> Result<Vec<u64>, sqlx::Error> {
        Ok(sqlx::query(
            "
                    SELECT id
                    FROM guilds
                ",
        )
        .fetch_all(&self.connection.pool)
        .await?
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.try_get::<u64, _>("id").unwrap())
        .collect())
    }
}
