use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "hehe no help 4 u >:)").await?;

    Ok(())
}
