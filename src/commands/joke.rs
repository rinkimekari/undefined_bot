use serde::Deserialize;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[allow(unused)]
const JOKE_URL: &'static str = "https://v2.jokeapi.dev/joke/Any";

#[derive(Deserialize)]
struct JokeFlags {
    nsfw: bool,
    religious: bool,
    political: bool,
    racist: bool,
    sexist: bool,
    explicit: bool,
}

#[derive(Deserialize)]
struct JokeData {
    error: bool,
    category: String,
    #[serde(rename = "type")]
    joke_type: String,
    setup: Option<String>,
    delivery: Option<String>,
    joke: Option<String>,
    flags: JokeFlags,
    id: u32,
    safe: bool,
    lang: String,
}

#[command]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Lmao").await?;

    Ok(())
}
