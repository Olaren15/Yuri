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

    pub async fn get_command_from_name_and_guild(
        &self,
        command_name: &str,
        guild_id: u64,
    ) -> Result<Command, sqlx::Error> {
        sqlx::query_as::<_, Command>(
            "
                SELECT *
                FROM commands
                    INNER JOIN command_guild
                    ON command_guild.command_id = commands.id
                WHERE commands.name = ?
                    AND command_guild.guild_id = ?
                LIMIT 1
                ",
        )
        .bind(command_name)
        .bind(guild_id)
        .fetch_one(&self.connection.pool)
        .await
    }

    pub async fn get_all_commands_in_guild(
        &self,
        guild_id: u64,
    ) -> Result<Vec<Command>, sqlx::Error> {
        sqlx::query_as::<_, Command>(
            "
                SELECT *
                FROM commands
                    INNER JOIN command_guild
                    ON command_guild.command_id = commands.id
                WHERE command_guild.guild_id = ?
                ORDER BY name
                ",
        )
        .bind(guild_id)
        .fetch_all(&self.connection.pool)
        .await
    }
}
