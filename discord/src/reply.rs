use common::models::command::Command;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::utils::Color;

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

    pub fn build_from_command(command: &Command, msg: &Message) -> Vec<Reply> {
        if msg.content.contains("everyone") {
            vec![Reply {
                message: msg.clone(),
                message_text: Reply::format_user_mentions(
                    command.everyone_text.as_str(),
                    &msg.author,
                    None,
                ),
                link: None,
            }]
        } else if msg.mentions.is_empty() {
            vec![Reply {
                message: msg.clone(),
                message_text: Reply::format_user_mentions(
                    command.nobody_text.as_str(),
                    &msg.author,
                    None,
                ),
                link: None,
            }]
        } else {
            msg.mentions
                .iter()
                .map(|mention: &User| Reply {
                    message: msg.clone(),
                    message_text: Reply::format_user_mentions(
                        command.one_person_text.as_str(),
                        &msg.author,
                        Some(&mention),
                    ),
                    link: None,
                })
                .collect()
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
