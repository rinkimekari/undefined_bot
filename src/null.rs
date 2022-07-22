use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::model::channel::Message;

const NULL_UUID: u64 = 699007266350694631;
const MEGAN_GIF: &'static str = "https://tenor.com/view/megan-tea-gif-17981754";

#[hook]
pub async fn null_message(ctx: &Context, msg: &Message) {
    if msg.author.id.0 != NULL_UUID {
        return;
    }

    if !msg.mention_everyone {
        return;
    }

    let _ = msg.reply(ctx, MEGAN_GIF).await;
}
