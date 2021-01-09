use crate::commands::reactions::Reaction;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::client::Context;

#[group]
#[commands(blush)]
struct Reactions;

#[command]
#[description("Get a blushing reaction!")]
async fn blush(ctx: &Context, msg: &Message) -> CommandResult {
    let reaction = Reaction {
        text: String::from("<@_s> is blushing"),
        images_file: Some(String::from("blushes.txt")),
    };

    reaction.build_reply(msg).send(&ctx, &msg).await;

    Ok(())
}
