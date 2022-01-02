use serde::Deserialize;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[allow(unused)]
const JOKE_URL: &'static str = "https://v2.jokeapi.dev/joke/Any";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct JokeFlags {
    nsfw: bool,
    religious: bool,
    political: bool,
    racist: bool,
    sexist: bool,
    explicit: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
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
    let joke_data = reqwest::get(JOKE_URL).await?.json::<JokeData>().await?;

    if joke_data.joke_type.as_str() == "single" {
        msg.reply(ctx, joke_data.joke.unwrap()).await?;
    } else {
        msg.reply(
            ctx,
            format!(
                "{}\n{}",
                joke_data.setup.unwrap(),
                joke_data.delivery.unwrap()
            ),
        )
        .await?;
    }

    Ok(())
}
