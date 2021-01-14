use crate::db_connection::DbConnection;
use crate::models::command::Command;
use sqlx::Row;

pub struct CommandRepository {
    connection: DbConnection,
}

impl CommandRepository {
    pub async fn new() -> CommandRepository {
        CommandRepository {
            connection: DbConnection::new().await,
        }
    }

    pub async fn get_command_from_name(&self, command_name: &str) -> Result<Command, sqlx::Error> {
        println!("{}", command_name);

        let row = sqlx::query(
            "
                SELECT *
                FROM commands
                WHERE name = ?
                LIMIT 1
        ",
        )
        .bind(command_name)
        .fetch_one(&self.connection.pool)
        .await?;

        let command = Command {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            everyone_text: row.try_get("everyone_text")?,
            nobody_text: row.try_get("nobody_text")?,
            one_person_text: row.try_get("one_person_text")?,
            is_nsfw: row.try_get("is_nsfw")?,
        };

        Ok(command)
    }
}
