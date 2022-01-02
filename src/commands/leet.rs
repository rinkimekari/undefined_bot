use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn leet(ctx: &Context, msg: &Message) -> CommandResult {
    let args = msg.content;
    println!("{}", args);
    let result = String::new();

    msg.reply(ctx, result).await?;

    Ok(())
}
