use serenity::model::gateway::Activity;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};

use common::db_connection::DbConnection;
use common::{models::settings::Settings, repositories::command_repository::CommandRepository};

use crate::built_in_commands::BuiltInCommands;
use crate::reply::Reply;

pub struct MessageHandler {
    pub settings: Settings,
    pub connection: DbConnection,
}

impl MessageHandler {
    pub fn extract_command_name(message_text: &str) -> &str {
        if let Some(first_space_index) = message_text.find(' ') {
            &message_text[1..first_space_index]
        } else {
            &message_text[1..]
        }
    }

    async fn handle_dynamic_commands(&self, ctx: &Context, msg: &Message) -> bool {
        if let Ok(command) = CommandRepository::new(&self.connection)
            .get_command_from_name(MessageHandler::extract_command_name(msg.content.as_str()))
            .await
        {
            for reply in Reply::from_command(&command, &msg, &self.connection).await {
                reply.send(&ctx).await;
            }

            return true;
        }

        false
    }
}

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg
            .content
            .starts_with(self.settings.command_prefix.as_str())
            && !BuiltInCommands::dispatch(&self.connection, &ctx, &msg).await
            && !self.handle_dynamic_commands(&ctx, &msg).await
        {
            Reply::from_str(
                &msg,
                format!(
                    "Command \"{}\" not recognized",
                    MessageHandler::extract_command_name(msg.content.as_str())
                )
                .as_str(),
            )
            .send(&ctx)
            .await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing("Being cute")).await;
        println!("{} is connected!", ready.user.name);
    }
}
