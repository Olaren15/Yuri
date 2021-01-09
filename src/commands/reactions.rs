pub(crate) mod normal;

use serenity::model::channel::Message;

use crate::image_repository::ImageRepository;
use crate::reply::Reply;

pub struct Reaction {
    pub text: String,
    pub images_file: Option<String>,
}

impl Reaction {
    pub fn build_reply(&self, msg: &Message) -> Reply {
        Reply {
            message: crate::commands::format_user_mentions(
                self.text.as_str(),
                &msg.author,
                msg.mentions.first(),
            ),
            link: if let Some(images_file) = self.images_file.as_ref() {
                ImageRepository::get_random_link_from_file(images_file.as_str())
            } else {
                None
            },
        }
    }
}
