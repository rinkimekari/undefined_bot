use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

const JOKE_URL: &'static str = "https://v2.jokeapi.dev/joke/Any";

#[command]
pub async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Lmao").await?;

    Ok(())
}
