use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::{channel::Message, id::UserId};

use serenity::model::user::User;
use serenity::utils::Color;
use std::{collections::HashSet, fs};

#[group]
#[commands(cuddle)]
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
    let messages: Vec<String> = if msg.content.contains("everyone") {
        vec![String::from("Cuddling everyone!")]
    } else if msg.mentions.is_empty() {
        vec![String::from(
            "Nobody to cuddle ;-;\nmention someone to cuddle them!",
        )]
    } else {
        msg.mentions
            .iter()
            .map(|mention: &User| format!("<@{}> is cuddling <@{}>", msg.author.id, mention.id))
            .collect()
    };

    for message in messages {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.description(message.as_str());
                    e.color(Color::MAGENTA);

                    e
                });

                m
            })
            .await?;
    }

    Ok(())
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
