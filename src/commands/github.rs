use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn github(ctx: &Context, msg: &Message) -> CommandResult {
    let response = "My source code is hosted at https://github.com/rinkimekari/undefined_bot.\n\
                    For potential contributors, please check the projects tab for good starting points.";
    msg.reply(ctx, response).await?;

    Ok(())
}
