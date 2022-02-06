use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

static HELP_MENU: &str = "```
!cat   --> Show a cat picture
!dog   --> Show a dog picture
!gnu   --> Give a wonderful speech by Mr. Libre himself
!help  --> Be rude to everyone except @rinkimekari#2514
!joke  --> Tell a (probably) offensive joke
!leet  --> B3cum a 1337 haxxor
!linux --> Give an even longer speech about freedom
!ping  --> Because we're lazy, here's the default ping-pong command
```";

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if *msg.author.id.as_u64() == 351141588455522315 {
        msg.reply(ctx, HELP_MENU).await?;
    } else {
        msg.reply(ctx, "hehe no help 4 u >:)").await?;
    }

    Ok(())
}
