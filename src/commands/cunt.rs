use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use std::thread::sleep;
use std::time::Duration;

#[command]
pub async fn cunt(ctx: &Context, msg: &Message) -> CommandResult {
    for _ in 0..15 {
        msg.reply(ctx, ":scottish:").await?;
        sleep(Duration::from_secs(2));
    }

    Ok(())
}
