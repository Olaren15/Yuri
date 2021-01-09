pub mod normal;
pub(crate) mod nsfw;

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
    pub fn build_replies(&self, msg: &Message) -> Vec<Reply> {
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
                    Reply {
                        message: crate::commands::format_user_mentions(
                            self.normal_text.as_str(),
                            &msg.author,
                            Some(mention),
                        ),
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
