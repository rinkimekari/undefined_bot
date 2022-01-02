use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;

use std::env;

mod commands;
use commands::GENERAL_GROUP;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let token = env::var("DISCORD_TOKEN").expect("Failed to get bot token");
    let prefix = env::var("BOT_PREFIX").expect("Failed to get discord bot prefix");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(prefix)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
