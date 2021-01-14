use common::models::settings::Settings;
use common::repositories::settings_repository::SettingsRepository;

use common::repositories::command_repository::CommandRepository;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler {
    settings: Settings,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg
            .content
            .starts_with(self.settings.command_prefix.as_str())
        {
            let command_name = if let Some(first_space_index) = msg.content.find(" ") {
                &msg.content[1..first_space_index]
            } else {
                &msg.content[1..]
            };

            if let Ok(command) = CommandRepository::new()
                .await
                .get_command_from_name(command_name)
                .await
            {
                msg.channel_id
                    .say(&ctx.http, command.everyone_text)
                    .await
                    .ok();
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {

        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let settings_repository = SettingsRepository::new().await;

    let settings = settings_repository
        .get_highest_weight_settings()
        .await
        .expect("Failed to retrieve settings from database");

    let handler = Handler { settings };

    let mut client = Client::builder(handler.settings.get_token_from_config())
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
