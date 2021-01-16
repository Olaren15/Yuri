use crate::db_connection::DbConnection;
use crate::models::command::Command;
use crate::models::image::Image;

pub struct ImageRepository {
    connection: DbConnection,
}

impl ImageRepository {
    pub fn new(connection: &DbConnection) -> ImageRepository {
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
}
