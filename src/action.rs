use serenity::model::{channel::Message, user::User};

use crate::image_repository::ImageRepository;
use crate::reply::Reply;

pub struct Action {
    pub everyone_text: String,
    pub nobody_text: String,
    pub normal_text: String,
    pub images_file: Option<String>,
}

impl Action {
    pub fn build_message(&self, msg: &Message) -> Vec<Reply> {
        if msg.content.contains("everyone") {
            vec![Reply {
                message: self.everyone_text.clone(),
                link: if let Some(images_file) = self.images_file.as_ref() {
                    ImageRepository::get_random_link_from_file(images_file.as_str())
                } else {
                    None
                },
            }]
        } else if msg.mentions.is_empty() {
            vec![Reply {
                message: self.nobody_text.clone(),
                link: None,
            }]
        } else {
            msg.mentions
                .iter()
                .map(|mention: &User| {
                    let mut formatted_message = self.normal_text.clone();

                    if let Some(sender_index) = self.normal_text.find("_s") {
                        formatted_message.replace_range(
                            sender_index..sender_index + 2,
                            msg.author.id.to_string().as_str(),
                        )
                    };

                    if let Some(receiver_index) = formatted_message.find("_r") {
                        formatted_message.replace_range(
                            receiver_index..receiver_index + 2,
                            mention.id.to_string().as_str(),
                        )
                    };

                    Reply {
                        message: formatted_message,
                        link: if let Some(images_file) = self.images_file.as_ref() {
                            ImageRepository::get_random_link_from_file(images_file.as_str())
                        } else {
                            None
                        },
                    }
                })
                .collect()
        }
    }
}
