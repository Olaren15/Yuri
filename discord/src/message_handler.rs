use crate::reply::Reply;

use common::{models::settings::Settings, repositories::command_repository::CommandRepository};

use common::db_connection::DbConnection;
use serenity::model::gateway::Activity;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};

pub struct MessageHandler {
    pub settings: Settings,
    pub connection: DbConnection,
}

impl MessageHandler {
    fn extract_command_name(message_text: &str) -> &str {
        if let Some(first_space_index) = message_text.find(" ") {
            &message_text[1..first_space_index]
        } else {
            &message_text[1..]
        }
    }
}

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg
            .content
            .starts_with(self.settings.command_prefix.as_str())
        {
            let command_name = MessageHandler::extract_command_name(msg.content.as_str());

            if let Ok(command) = CommandRepository::new(&self.connection)
                .get_command_from_name(command_name)
                .await
            {
                for reply in Reply::build_from_command(&command, &msg) {
                    reply.send(&ctx).await;
                }
            } else {
                msg.reply(ctx, format!("Command \"{}\" not recognized", command_name))
                    .await
                    .ok();
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing("Being cute")).await;
        println!("{} is connected!", ready.user.name);
    }
}
