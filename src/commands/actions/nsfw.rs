use crate::commands::actions::Action;
use crate::commands::reactions::Reaction;

use serenity::client::Context;
use serenity::framework::standard::macros::{check, command, group};
use serenity::framework::standard::CheckResult::{Failure, Success};
use serenity::framework::standard::{Args, CheckResult, CommandOptions, CommandResult, Reason};
use serenity::model::channel::Message;

#[group]
#[commands(spank, tease)]
struct NsfwActions;

#[check]
#[name = "horny"]
async fn horny_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    let channel = msg.channel_id.to_channel(&ctx).await.unwrap();
    return if channel.is_nsfw() {
        Success
    } else {
        // don't show horny jail if checking with the help command
        if !msg.content.contains("help") {
            let reaction = Reaction {
                text: String::from("This is not an nsfw channel >:("),
                images_file: Some(String::from("bonks.txt")),
            };

            reaction.build_reply(msg).send(&ctx, &msg).await;
        }

        Failure(Reason::User(String::from("H O R N Y")))
    };
}

#[command]
#[checks(horny)]
#[description = "Spank someone by mentioning them!"]
async fn spank(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Spanking everyone :smiling_imp:"),
        nobody_text: String::from("Nobody to spank ;-;\nmention someone to spank them!"),
        normal_text: String::from("<@_s> spanked <@_r>!"),
        images_file: Some(String::from("spanks.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[checks(horny)]
#[description = "Tease someone by mentioning them!"]
async fn tease(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Teasing everyone :smirk:"),
        nobody_text: String::from("Nobody to tease ;-;\nmention someone to tease them!"),
        normal_text: String::from("<@_s> teased <@_r>!"),
        images_file: Some(String::from("teases.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}
