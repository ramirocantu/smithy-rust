use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use std::env;
use std::io::Read;

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(
        &ctx.http,
        "Type -start to start the ftb server\nType -help to see this message ",
    )?;

    Ok(())
}

#[command]
fn start(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut res = reqwest::blocking::get(&env::var("API_START").expect("API_START missing"))?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    msg.channel_id.say(
        &ctx.http,
        format!(
            "Sent start request to server mc.ramirocantu.me\n Received response: {}",
            res.status()
        ),
    )?;
    //TODO Build response on gcloud api

    Ok(())
}

#[group]
#[commands(help, start)]
struct General;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("-")) // set the bot's prefix to "-"
            .group(&GENERAL_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
