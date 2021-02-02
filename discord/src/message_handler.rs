use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::{Message, Reaction},
        gateway::{Activity, Ready},
    },
};

use common::db_connection::DbConnection;
use common::models::command::Command;
use common::repositories::image_repository::ImageRepository;
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
            if !Self::check_nsfw_and_bonk(&command, ctx, msg, &self.connection).await {
                for reply in Reply::from_command(&command, &msg, &self.connection).await {
                    reply.send(&ctx).await;
                }
            }

            return true;
        }

        false
    }

    pub async fn check_nsfw_and_bonk(
        cmd: &Command,
        ctx: &Context,
        msg: &Message,
        conn: &DbConnection,
    ) -> bool {
        let should_bonk = if cmd.is_nsfw {
            if let Ok(channel) = msg.channel_id.to_channel(ctx).await {
                !channel.is_nsfw()
            } else {
                false
            }
        } else {
            false
        };

        if should_bonk {
            let mut reply = Reply::from_str(msg, "This is not an nsfw channel >:(");
            reply.link = ImageRepository::new(conn)
                .get_random_link_from_command_name("bonk")
                .await
                .ok();

            reply.send(ctx).await;
        }

        should_bonk
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

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Ok(msg) = reaction.message(&ctx).await {
            if !msg.embeds.is_empty() {
                if let Ok(app_info) = ctx.http.get_current_application_info().await {
                    if msg.author.id == app_info.id {
                        // we know that the message is sent from yuri
                        if let Some(reaction_user_id) = reaction.user_id {
                            if reaction_user_id != app_info.id {
                                // we know that the reaction is not from yuri

                                if let Some(description) = &msg.embeds[0].description {
                                    if description.contains("Do you accept?") {
                                        if BuiltInCommands::accept(
                                            &self.connection,
                                            &ctx,
                                            &msg,
                                            &reaction,
                                        )
                                        .await
                                        {
                                            msg.delete(ctx).await.ok();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing(
            format!("{}help", self.settings.command_prefix).as_str(),
        ))
        .await;
        println!("{} is connected!", ready.user.name);
    }
}
