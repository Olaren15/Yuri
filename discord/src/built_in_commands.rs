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
            "accept" => Self::accept(conn, ctx, msg).await,
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
                        e.description("Hello comrade! Here are your gay commands

Strikethrough commands are unavailable because they require to be in an nsfw channel.");
                        e.color(Color::MAGENTA);

                        e.field("`help`", "Show this help message", false);
                        e.field("`offer`", "Offer a command to someone before touching them", true);
                        e.field("`accept`", "Accept someone's offer.\nMust be used when replying to an offer message", true);

                        e.fields(
                            commands
                                .into_iter()
                                .map(|command| {
                                    if command.is_nsfw && !channel_is_nsfw {
                                        (format!("~~`{}`~~", command.name), command.description, false)
                                    } else {
                                        (format!("`{}`", command.name), command.description, false)
                                    }
                                }),
                        );


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

        Reply::from_str(msg, message.as_str()).send(ctx).await
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
                    id: 0, // dummy id
                    name: String::from(""), // dummy name
                    description: String::from(""), // dummy description
                    everyone_text: String::from(
                        "Offering to everyone is not supported at the moment.",
                    ),
                    nobody_text: format!(
                        "mention someone to offer them `{}`",
                        offered_command.name
                    ),
                    one_person_text: format!(
                        "Offering `{}` to <@_r> \n\nReply to this message with the `accept` command to accept",
                        offered_command.name
                    ),
                    is_nsfw: offered_command.is_nsfw,
                };

                for reply in Reply::from_command(&command, msg, conn).await {
                    reply.send(ctx).await;
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
    async fn accept(conn: &DbConnection, ctx: &Context, msg: &Message) {
        if let Some(reference) = &msg.message_reference {
            if let Some(message_id) = reference.message_id {
                if let Ok(ref_message) = ctx
                    .http
                    .get_message(reference.channel_id.into(), message_id.into())
                    .await
                {
                    if !ref_message.embeds.is_empty() {
                        if let Some(text) = &ref_message.embeds[0].description {
                            if text.contains(format!("<@{}>", msg.author.id).as_str()) {
                                // extract data from message
                                if let Some(command_start) = text.find('`') {
                                    if let Some(command_length) =
                                        text[command_start + 1..].find('`')
                                    {
                                        let command = &text
                                            [command_start + 1..command_start + command_length + 1];

                                        if let Some(sender_start) = text.find("<@") {
                                            if let Some(sender_length) =
                                                text[sender_start + 2..].find('>')
                                            {
                                                let sender = &text[sender_start + 2
                                                    ..sender_start + 2 + sender_length];

                                                // build the message
                                                if let Ok(mut command) =
                                                    CommandRepository::new(conn)
                                                        .get_command_from_name(command)
                                                        .await
                                                {
                                                    // manually replace the sender and the receiver
                                                    if let Some(sender_index) =
                                                        command.one_person_text.find("_s")
                                                    {
                                                        command.one_person_text.replace_range(
                                                            sender_index..sender_index + 2,
                                                            sender,
                                                        );
                                                    }

                                                    if let Some(receiver_index) =
                                                        command.one_person_text.find("_r")
                                                    {
                                                        command.one_person_text.replace_range(
                                                            receiver_index..receiver_index + 2,
                                                            msg.author.id.to_string().as_str(),
                                                        );
                                                    }

                                                    for reply in
                                                        Reply::from_command(&command, msg, conn)
                                                            .await
                                                    {
                                                        reply.send(ctx).await;
                                                    }

                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                Reply::from_str(msg, "This offer isn't for you!")
                                    .send(ctx)
                                    .await;
                                return;
                            }
                        }
                    }
                }
            }
        }

        Reply::from_str(msg, "Offer not found.\n\nReply to an offer to accept it")
            .send(ctx)
            .await;
    }
}
