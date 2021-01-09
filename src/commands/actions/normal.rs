use crate::commands::actions::Action;

use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message};

#[group]
#[commands(cuddle, hug, sad_hug, pat, kiss, slap, handhold, bonk)]
struct Actions;

#[command]
#[description = "Cuddle someone by mentioning them!"]
async fn cuddle(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Cuddling everyone"),
        nobody_text: String::from("Nobody to cuddle ;-;\nmention someone to cuddle them!"),
        normal_text: String::from("<@_s> is cuddling <@_r>"),
        images_file: Some(String::from("cuddles.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[description = "Hug someone by mentioning them!"]
async fn hug(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Group hug!!!"),
        nobody_text: String::from("Nobody to hug ;-;\nmention someone to hug them!"),
        normal_text: String::from("<@_s> gave <@_r> a hug"),
        images_file: Some(String::from("hugs.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[description = "Give a sad hug someone by mentioning them"]
async fn sad_hug(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Sad group hug :sob:"),
        nobody_text: String::from(
            "Nobody to give sad hugs, good news!\nmention someone to give them a sad hug",
        ),
        normal_text: String::from("<@_s> gave <@_r> a hug"),
        images_file: Some(String::from("sad-hugs.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[description = "Pat someone by mentioning them!"]
async fn pat(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Patting everyone!!!"),
        nobody_text: String::from("Nobody to pat ;-;\nmention someone to pat them!"),
        normal_text: String::from("<@_s> is patting <@_r>"),
        images_file: Some(String::from("pats.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[description = "Kiss someone by mentioning them!"]
async fn kiss(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Kissing everyone :flushed: :flushed: :flushed:"),
        nobody_text: String::from("Nobody to kiss ;-;\nmention someone to kiss them!"),
        normal_text: String::from("<@_s> gave <@_r> a kiss :flushed:"),
        images_file: Some(String::from("kisses.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[description = "Slap someone by mentioning them!"]
async fn slap(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Slapping everyone!"),
        nobody_text: String::from("Nobody to slap ;-;\nmention someone to slap them!"),
        normal_text: String::from("<@_s> slapped <@_r>"),
        images_file: Some(String::from("slaps.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[description = "Hold someone's hand by mentioning them!"]
async fn handhold(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Holding everyone's hands!"),
        nobody_text: String::from("Nobody to hold hands with ;-;\nmention someone to hold their hand!"),
        normal_text: String::from("<@_s> held <@_r>'s hand :flushed:"),
        images_file: Some(String::from("handholds.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

#[command]
#[description = "Bonk someone by mentioning them!"]
async fn bonk(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Bonking everyone!"),
        nobody_text: String::from("Nobody to bonk ;-;\nmention someone to bonk them!"),
        normal_text: String::from("<@_s> bonked <@_r>"),
        images_file: Some(String::from("bonks.txt")),
    };

    for replies in action.build_replies(msg) {
        replies.send(&ctx, &msg).await;
    }

    Ok(())
}

