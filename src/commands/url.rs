use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn url(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Not made yet.").await?;

    Ok(())
}
