mod message_handler;
mod reply;

use crate::message_handler::MessageHandler;
use common::db_connection::DbConnection;
use common::repositories::settings_repository::SettingsRepository;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    let connection = DbConnection::new().await;

    let settings = SettingsRepository::new(&connection)
        .get_highest_weight_settings()
        .await
        .expect("Failed to retrieve settings from database");

    let handler = MessageHandler {
        settings,
        connection,
    };

    if let Err(why) = Client::builder(handler.settings.get_token_from_config())
        .event_handler(handler)
        .await
        .expect("Err creating client")
        .start()
        .await
    {
        println!("Client error: {:?}", why);
    }
}
