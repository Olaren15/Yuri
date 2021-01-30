use serenity::{client::Context, model::channel::Message};

use common::db_connection::DbConnection;
use common::repositories::command_repository::CommandRepository;
use common::repositories::guild_repository::GuildRepository;

use crate::message_handler::MessageHandler;
use crate::reply::Reply;

pub struct BuiltInCommands;

impl BuiltInCommands {
    pub async fn dispatch(conn: &DbConnection, ctx: &Context, msg: &Message) -> bool {
        match MessageHandler::extract_command_name(msg.content.as_str()) {
            "help" => Self::help(conn, ctx, msg).await,
            "register" => Self::register_guild(conn, ctx, msg).await,
            _ => {
                return false;
            }
        }

        true
    }

    async fn help(conn: &DbConnection, ctx: &Context, msg: &Message) {
        let command_text =
            if let Ok(commands) = CommandRepository::new(conn).get_all_commands().await {
                commands
                    .iter()
                    .map(|command| format!("{}\n", command.name))
                    .collect()
            } else {
                String::from("")
            };

        Reply::from_str(msg, command_text.as_str()).send(ctx).await;
    }

    async fn register_guild(conn: &DbConnection, ctx: &Context, msg: &Message) {
        let message;
        if let Some(guild_id) = msg.guild_id {
            match GuildRepository::new(&conn)
                .register_guild(*guild_id.as_u64())
                .await
            {
                Ok(_) => message = String::from("Successfully registered this server! You can now configure it using the web interface!"),
                Err(msg) => message = msg.clone(),
            }
        } else {
            message = String::from("Sorry, I can't register DM channels.");
        }

        Reply::from_str(msg, message.as_str()).send(ctx).await
    }
}
