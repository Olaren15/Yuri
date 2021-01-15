mod message_handler;
mod reply;

use common::repositories::settings_repository::SettingsRepository;
use crate::message_handler::MessageHandler;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    let settings_repository = SettingsRepository::new().await;

    let settings = settings_repository
        .get_highest_weight_settings()
        .await
        .expect("Failed to retrieve settings from database");

    let handler = MessageHandler { settings };

    let mut client = Client::builder(handler.settings.get_token_from_config())
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
