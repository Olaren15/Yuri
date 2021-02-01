use crate::{db_connection::DbConnection, models::command::Command};

pub struct CommandRepository {
    connection: DbConnection,
}

impl CommandRepository {
    pub fn new(connection: &DbConnection) -> CommandRepository {
        CommandRepository {
            connection: connection.clone(),
        }
    }

    pub async fn get_command_from_name(&self, command_name: &str) -> Result<Command, sqlx::Error> {
        sqlx::query_as::<_, Command>(
            "
                SELECT *
                FROM commands
                WHERE name = ?
                LIMIT 1
                ",
        )
        .bind(command_name)
        .fetch_one(&self.connection.pool)
        .await
    }

    pub async fn get_all_commands(&self) -> Result<Vec<Command>, sqlx::Error> {
        sqlx::query_as::<_, Command>(
            "
                SELECT *
                FROM commands
                ORDER BY name
                ",
        )
        .fetch_all(&self.connection.pool)
        .await
    }
}
