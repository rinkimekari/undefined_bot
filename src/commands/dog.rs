use serde::Deserialize;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

const DOG_API: &str = "https://dog.ceo/api/breeds/image/random";

#[allow(dead_code)]
#[derive(Deserialize)]
struct DogData {
    message: String,
    status: String,
}

#[command]
pub async fn dog(ctx: &Context, msg: &Message) -> CommandResult {
    let dog_data = reqwest::get(DOG_API).await?.json::<DogData>().await?;

    let mut reply = CreateEmbed::default();
    reply.title("Look at this doggo!");
    reply.color((0, 0xff, 0));
    reply.image(dog_data.message);

    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(reply.clone()))
        .await?;
    Ok(())
}
