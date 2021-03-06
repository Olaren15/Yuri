use crate::db_conntext::DbContext;
use crate::models::command::Command;
use crate::models::image::Image;

pub struct ImageRepository {
    connection: DbContext,
}

impl ImageRepository {
    pub fn new(connection: &DbContext) -> ImageRepository {
        ImageRepository {
            connection: connection.clone(),
        }
    }

    pub async fn get_random_link_from_command(
        &self,
        command: &Command,
    ) -> Result<String, sqlx::Error> {
        let image = sqlx::query_as::<_, Image>(
            "
                SELECT *
                FROM images
                WHERE command_id = ?
                ORDER BY RAND()  
                ",
        )
        .bind(command.id)
        .fetch_one(&self.connection.pool)
        .await?;

        Ok(image.url)
    }

    pub async fn get_random_link_from_command_name(
        &self,
        name: &str,
    ) -> Result<String, sqlx::Error> {
        let image = sqlx::query_as::<_, Image>(
            "
                SELECT *
                FROM images
                INNER JOIN commands
                    ON commands.id = command_id
                WHERE commands.name = ?
                ORDER BY RAND()  
                ",
        )
        .bind(name)
        .fetch_one(&self.connection.pool)
        .await?;

        Ok(image.url)
    }
}
