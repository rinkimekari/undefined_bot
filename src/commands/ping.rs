use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let now = std::time::SystemTime::now();
    let mut message = msg.reply(ctx, "Pong!").await?;

    let message_time = now.elapsed()?.as_millis();
    message.edit(ctx, |m| m.content(format!("Pong! Took {message_time}ms"))).await?;

    Ok(())
}
