use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let prefix = std::env::var("BOT_PREFIX").unwrap(); // cant be err cuz main
    let help_menu = format!("```
{prefix}help       --> Be rude to everyone except @rinkimekari#2514
{prefix}ping       --> Because we're lazy, here's the default ping-pong command
{prefix}cat        --> Show a cat picture
{prefix}dog        --> Show a dog picture
{prefix}gnu        --> Give a wonderful speech by Mr. Libre himself
{prefix}linux      --> Give an even longer speech about freedom
{prefix}joke       --> Tell a (probably) offensive joke
{prefix}leet       --> B3cum a 1337 haxxor
{prefix}ip         --> Scan that sus IP like a pro hecker
{prefix}url        --> Be less of a pro hecker and scan a URL !!! NOT MADE YET !!!
{prefix}virustotal --> *Insert Ghost-Busters theme song, but for VirusTotal* !!! NOT MADE YET !!!
```");

    if *msg.author.id.as_u64() == 351141588455522315 {
        msg.reply(ctx, help_menu).await?;
    } else {
        msg.reply(ctx, "hehe no help 4 u >:)").await?;
    }

    Ok(())
}
