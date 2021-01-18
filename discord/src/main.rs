use serenity::prelude::*;

use common::db_connection::DbConnection;
use common::repositories::settings_repository::SettingsRepository;

use crate::message_handler::MessageHandler;

mod message_handler;
mod reply;

#[tokio::main]
async fn main() -> serenity::Result<()> {
    let connection = DbConnection::new().await;

    let settings = SettingsRepository::new(&connection)
        .get_highest_weight_settings()
        .await
        .expect("Failed to retrieve settings from database");

    let handler = MessageHandler {
        settings,
        connection,
    };

    Client::builder(handler.settings.bot_token.as_str())
        .event_handler(handler)
        .await
        .expect("Err creating client")
        .start()
        .await
}
