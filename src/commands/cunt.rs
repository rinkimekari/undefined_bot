use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn cunt(ctx: &Context, msg: &Message) -> CommandResult {
    let emoji = ctx
        .http
        .get_emoji(927022168963948595, 999846296745680906)
        .await
        .unwrap();

    msg.reply(ctx, &emoji).await?;

    Ok(())
}
