use serenity::model::{channel::Message, user::User};

pub struct Action {
    pub everyone_text: String,
    pub nobody_text: String,
    pub normal_text: String,
}

impl Action {
    pub fn build_messages(&self, msg: &Message) -> Vec<String> {
        if msg.content.contains("everyone") {
            vec![self.everyone_text.clone()]
        } else if msg.mentions.is_empty() {
            vec![self.nobody_text.clone()]
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

                    formatted_message
                })
                .collect()
        }
    }
}
