use futures::executor;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::utils::Color;

use common::db_connection::DbConnection;
use common::models::command::Command;
use common::repositories::image_repository::ImageRepository;

pub struct Reply {
    message: Message,
    pub message_text: String,
    pub link: Option<String>,
}

impl Reply {
    fn format_user_mentions(raw_message: &str, sender: &User, receiver: Option<&User>) -> String {
        let mut formatted_message = raw_message.to_string();

        if let Some(sender_index) = formatted_message.find("_s") {
            formatted_message.replace_range(
                sender_index..sender_index + 2,
                sender.id.to_string().as_str(),
            )
        };

        if let (Some(receiver), Some(receiver_index)) = (receiver, formatted_message.find("_r")) {
            formatted_message.replace_range(
                receiver_index..receiver_index + 2,
                receiver.id.to_string().as_str(),
            )
        };

        formatted_message
    }

    pub async fn from_command(
        command: &Command,
        msg: &Message,
        connection: &DbConnection,
    ) -> Vec<Reply> {
        if msg.content.contains("everyone") {
            vec![Reply {
                message: msg.clone(),
                message_text: Reply::format_user_mentions(
                    command.everyone_text.as_str(),
                    &msg.author,
                    None,
                ),
                link: ImageRepository::new(connection)
                    .get_random_link_from_command(&command)
                    .await
                    .ok(),
            }]
        } else if msg.mentions.is_empty() {
            vec![Reply {
                message: msg.clone(),
                message_text: Reply::format_user_mentions(
                    command.nobody_text.as_str(),
                    &msg.author,
                    None,
                ),
                link: ImageRepository::new(connection)
                    .get_random_link_from_command(&command)
                    .await
                    .ok(),
            }]
        } else {
            msg.mentions
                .iter()
                .map(|mention| {
                    executor::block_on(async {
                        Reply {
                            message: msg.clone(),
                            message_text: Reply::format_user_mentions(
                                command.one_person_text.as_str(),
                                &msg.author,
                                Some(&mention),
                            ),
                            link: ImageRepository::new(connection)
                                .get_random_link_from_command(&command)
                                .await
                                .ok(),
                        }
                    })
                })
                .collect()
        }
    }

    pub fn from_str(msg: &Message, text: &str) -> Reply {
        Reply {
            message: msg.clone(),
            message_text: text.to_string(),
            link: None,
        }
    }

    pub async fn send(&self, ctx: &Context) {
        self.message
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.description(self.message_text.as_str());
                    e.color(Color::MAGENTA);
                    if let Some(link) = self.link.as_ref() {
                        e.image(link);
                    }

                    e
                });
                m
            })
            .await
            .unwrap();
    }
}
