use crate::reply::Reply;

use common::{models::settings::Settings, repositories::command_repository::CommandRepository};

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};

pub struct MessageHandler {
    pub settings: Settings,
}

#[async_trait]
impl EventHandler for MessageHandler {
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
                for reply in Reply::build_from_command(&command, &msg) {
                    reply.send(&ctx).await;
                }
            } else {
                msg.reply(ctx, format!("Command \"{}\" not recognized", command_name)).await.ok();
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
