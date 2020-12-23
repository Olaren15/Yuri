mod actions;

use actions::Action;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::{channel::Message, id::UserId};

use serenity::model::id::ChannelId;
use serenity::utils::Color;
use std::{collections::HashSet, fs};

#[group]
#[commands(cuddle, hug, pat, kiss)]
struct Cute;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&CUTE_GROUP)
        .help(&MY_HELP);

    let token = fs::read_to_string("token.txt")
        .expect("Failed to read token from file. Make sure that the file 'token.txt' is valid");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
#[bucket = "cute"]
#[description = "Cuddle someone by mentioning them!"]
async fn cuddle(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Cuddling everyone"),
        nobody_text: String::from("Nobody to cuddle ;-;\nmention someone to cuddle them!"),
        normal_text: String::from("<@_s> is cuddling <@_r>"),
    };

    send_messages(&ctx, msg.channel_id, &action.build_messages(msg)).await;

    Ok(())
}

#[command]
#[bucket = "cute"]
#[description = "Hug someone by mentioning them!"]
async fn hug(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Group hug!!!"),
        nobody_text: String::from("Nobody to hug ;-;\nmention someone to hug them!"),
        normal_text: String::from("<@_s> gave <@_r> a hug"),
    };

    send_messages(&ctx, msg.channel_id, &action.build_messages(msg)).await;

    Ok(())
}

#[command]
#[bucket = "cute"]
#[description = "Pat someone by mentioning them!"]
async fn pat(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Patting everyone!!!"),
        nobody_text: String::from("Nobody to pat ;-;\nmention someone to pat them!"),
        normal_text: String::from("<@_s> is patting <@_r>"),
    };

    send_messages(&ctx, msg.channel_id, &action.build_messages(msg)).await;

    Ok(())
}

#[command]
#[bucket = "cute"]
#[description = "Kiss someone by mentioning them!"]
async fn kiss(ctx: &Context, msg: &Message) -> CommandResult {
    let action = Action {
        everyone_text: String::from("Kissing everyone :flushed: :flushed: :flushed:"),
        nobody_text: String::from("Nobody to kiss ;-;\nmention someone to kiss them!"),
        normal_text: String::from("<@_s> gave <@_r> a kiss :flushed:"),
    };

    send_messages(&ctx, msg.channel_id, &action.build_messages(msg)).await;

    Ok(())
}

async fn send_messages(ctx: &Context, channel_id: ChannelId, messages: &Vec<String>) {
    for message in messages {
        channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.description(message.as_str());
                    e.color(Color::MAGENTA);

                    e
                });

                m
            })
            .await
            .unwrap();
    }
}

#[help]
#[individual_command_tip = "Hello!\n\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Hide"]
#[wrong_channel = "Hide"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
