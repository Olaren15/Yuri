use serenity::model::channel::ReactionType::Unicode;
use serenity::model::channel::{Reaction, ReactionType};
use serenity::utils::Color;
use serenity::{client::Context, model::channel::Message};

use common::db_connection::DbConnection;
use common::models::command::Command;
use common::repositories::command_repository::CommandRepository;
use common::repositories::guild_repository::GuildRepository;

use crate::message_handler::MessageHandler;
use crate::reply::Reply;

pub struct BuiltInCommands;

impl BuiltInCommands {
    pub async fn dispatch(conn: &DbConnection, ctx: &Context, msg: &Message) -> bool {
        match MessageHandler::extract_command_name(msg.content.as_str()) {
            "help" => Self::help(conn, ctx, msg).await,
            "register" => Self::register(conn, ctx, msg).await,
            "offer" => Self::offer(conn, ctx, msg).await,
            _ => {
                return false;
            }
        }

        true
    }

    async fn help(conn: &DbConnection, ctx: &Context, msg: &Message) {
        if let Ok(commands) = CommandRepository::new(conn).get_all_commands().await {
            let channel_is_nsfw = if let Ok(channel) = msg.channel_id.to_channel(ctx).await {
                channel.is_nsfw()
            } else {
                false
            };

            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.description(
                            "Hello comrade! Here are your gay commands

Strikethrough commands are unavailable because they require to be in an nsfw channel.",
                        );
                        e.color(Color::MAGENTA);

                        e.field("`help`", "Show this help message", false);
                        e.field(
                            "`offer`",
                            "Offer a command to someone before touching them",
                            true,
                        );

                        e.fields(commands.into_iter().map(|command| {
                            if command.is_nsfw && !channel_is_nsfw {
                                (
                                    format!("~~`{}`~~", command.name),
                                    command.description,
                                    false,
                                )
                            } else {
                                (format!("`{}`", command.name), command.description, false)
                            }
                        }));

                        e
                    });

                    m
                })
                .await
                .unwrap();
        }
    }

    async fn register(conn: &DbConnection, ctx: &Context, msg: &Message) {
        let message;
        if let Some(guild_id) = msg.guild_id {
            match GuildRepository::new(&conn)
                .register_guild(*guild_id.as_u64())
                .await
            {
                Ok(_) => message = String::from("Successfully registered this server! You can now configure it using the web interface!"),
                Err(msg) => message = msg,
            }
        } else {
            message = String::from("Sorry, I can't register DM channels.");
        }

        Reply::from_str(msg, message.as_str()).send(ctx).await;
    }

    async fn offer(conn: &DbConnection, ctx: &Context, msg: &Message) {
        if let Some(index) = msg.content.find(' ') {
            let offer = &msg.content[index + 1..];

            let offer_name = if let Some(space_index) = offer.find(' ') {
                &offer[..space_index]
            } else {
                offer
            };

            if let Ok(offered_command) = CommandRepository::new(conn)
                .get_command_from_name(offer_name)
                .await
            {
                let command = Command {
                    id: 0,                         // dummy id
                    name: String::from(""),        // dummy name
                    description: String::from(""), // dummy description
                    everyone_text: String::from(
                        "Offering to everyone is not supported at the moment.",
                    ),
                    nobody_text: format!(
                        "mention someone to offer them `{}`",
                        offered_command.name
                    ),
                    one_person_text: format!(
                        "<@_s> is offering `{}` to <@_r> \n\nDo you accept?",
                        offered_command.name
                    ),
                    is_nsfw: offered_command.is_nsfw,
                };

                for reply in Reply::from_command(&command, msg, conn).await {
                    let message = reply.send(ctx).await;
                    message
                        .react(ctx, ReactionType::Unicode(String::from("✅")))
                        .await
                        .ok();

                    message
                        .react(ctx, ReactionType::Unicode(String::from("❌")))
                        .await
                        .ok();
                }
            } else {
                Reply::from_str(msg, format!("command not found: {}", offer_name).as_str())
                    .send(ctx)
                    .await;
            }
        } else {
            Reply::from_str(msg, "nothing to offer\n\nUsage: `offer <command> <@user>`")
                .send(ctx)
                .await;
        }
    }

    // Code is ugly, but it works soooooo it's probably gonna stay like this
    pub async fn accept(
        conn: &DbConnection,
        ctx: &Context,
        msg: &Message,
        reaction: &Reaction,
    ) -> bool {
        if let Unicode(emoji) = &reaction.emoji {
            if let Some(text) = &msg.embeds[0].description {
                // start of sender & receiver parsing
                if let Some(sender_begin_index) = text.find("<@") {
                    let sender_begin_index = sender_begin_index + 2;
                    if let Some(sender_length) = &text[sender_begin_index..].find(">") {
                        let sender_id =
                            &text[sender_begin_index..sender_begin_index + sender_length];

                        if let Some(receiver_begin_index) = text[sender_begin_index..].find("<@") {
                            let receiver_begin_index =
                                receiver_begin_index + sender_begin_index + 2;
                            if let Some(receiver_length) = text[receiver_begin_index..].find(">") {
                                let receiver_id = &text
                                    [receiver_begin_index..receiver_begin_index + receiver_length];
                                // sender & receiver parsed

                                if let Some(reaction_user_id) = reaction.user_id {
                                    if receiver_id.parse::<u64>().unwrap_or_default()
                                        == *reaction_user_id.as_u64()
                                    {
                                        // person who reacted is the receiver
                                        if emoji.contains("✅") {
                                            // user accepts

                                            if let Some(command_begin_index) = text.find('`') {
                                                let command_begin_index = command_begin_index + 1;

                                                if let Some(command_length) =
                                                    &text[command_begin_index..].find('`')
                                                {
                                                    let command_name = &text[command_begin_index
                                                        ..command_begin_index + command_length];

                                                    if let Ok(mut command) =
                                                        CommandRepository::new(conn)
                                                            .get_command_from_name(command_name)
                                                            .await
                                                    {
                                                        if let Some(sender_index) =
                                                            command.one_person_text.find("_s")
                                                        {
                                                            command.one_person_text.replace_range(
                                                                sender_index..sender_index + 2,
                                                                sender_id,
                                                            );
                                                        }

                                                        if let Some(receiver_index) =
                                                            command.one_person_text.find("_r")
                                                        {
                                                            command.one_person_text.replace_range(
                                                                receiver_index..receiver_index + 2,
                                                                receiver_id,
                                                            );
                                                        }

                                                        Reply::from_command_offer(
                                                            &command, &msg, &conn,
                                                        )
                                                        .await
                                                        .send(ctx)
                                                        .await;

                                                        return true;
                                                    }
                                                }
                                            }
                                        } else if emoji.contains("❌") {
                                            Reply::from_str(
                                                &msg,
                                                format!(
                                                    "<@{}> refused <@{}>'s offer",
                                                    receiver_id, sender_id
                                                )
                                                .as_str(),
                                            )
                                            .send(ctx)
                                            .await;

                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}
