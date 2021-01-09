pub(crate) mod actions;
pub(crate) mod reactions;

use serenity::model::user::User;

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
