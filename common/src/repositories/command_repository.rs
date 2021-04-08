use crate::{db_conntext::DbContext, models::command::Command};

pub struct CommandRepository {
    connection: DbContext,
}

impl CommandRepository {
    pub fn new(connection: &DbContext) -> CommandRepository {
        CommandRepository {
            connection: connection.clone(),
        }
    }

    pub async fn get_command_from_name(&self, command_name: &str) -> Result<Command, sqlx::Error> {
        sqlx::query_as::<_, Command>(
            "
                SELECT *
                FROM commands
                    INNER JOIN command_guild
                    ON command_guild.command_id = commands.id
                WHERE commands.name = ?
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
