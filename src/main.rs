mod action;
mod cute;
mod image_provider;
mod reply;

use serenity::client::{Client, Context};
use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::{channel::Message, id::UserId};

use std::{collections::HashSet, fs};

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&crate::cute::CUTESTUFF_GROUP)
        .help(&MY_HELP);

    let token = fs::read_to_string("token.txt")
        .expect("Failed to read token from file. Make sure that the file 'token.txt' is valid");

    let mut client = Client::builder(token)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
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
