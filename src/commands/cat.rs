use rand::{thread_rng, Rng};
use serde::Deserialize;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

const CAT_API: [&'static str; 2] = [
    "https://api.thecatapi.com/api/images/get?format=json&type=gif",
    "https://api.thecatapi.com/api/images/get?format=json",
];

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CatData {
    id: String,
    url: String,
    source_url: String,
}

#[command]
pub async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let idx = thread_rng().gen_range(0..=1);
    let cat_data = reqwest::get(CAT_API[idx])
        .await?
        .json::<Vec<CatData>>()
        .await?;

    let mut reply = CreateEmbed::default();
    reply.title("Cat!");
    reply.color((240, 0, 35));
    reply.image(&cat_data[0].url);

    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(reply.clone()))
        .await?;
    Ok(())
}
