mod parsing;
use std::env;

use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::{channel::Message};
use serenity::prelude::*;

#[group]
#[commands(lyr_url, lyr)]
struct General;

struct Handler;

#[command]
async fn lyr_url(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args.single_quoted::<String>().unwrap();
    match parsing::get_lyrics_url(&url) {
        Ok(lyrics) => {
            msg.reply(ctx, lyrics).await?;
        },
        Err(e) => {
            let error_message = format!("Произошла ошибка: {}", e);
            let sendable_error = Box::new(error_message) as Box<dyn std::error::Error + Send + Sync>;
            msg.reply(ctx, sendable_error).await?;
        }
    Ok(())
}

#[command]
async fn lyr(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let performer = args.single_quoted::<String>().unwrap();
    let title = args.single_quoted::<String>().unwrap();
    match parsing::get_lyrics(&performer, &title) {
        Ok(lyrics) => {
            msg.reply(ctx, lyrics).await?;
        },
        Err(e) => {
            msg.reply(ctx, format!("Произошла ошибка: {}", e)).await?;
        }
    }
    Ok(())
}

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
