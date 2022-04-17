use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let now = std::time::SystemTime::now(); // Get the current time
    let message = msg.reply(ctx, "Pong!").await?; // Send initial message responce
    let message_time = now.elapsed().unwrap(); // Get the time it took to send the message

    message.edit(ctx, |m| m.content(format!("Pong! Took {}ms", message_time.as_millis()))).await?; // Edit the message to show the time it took to send it

    Ok(())
}
