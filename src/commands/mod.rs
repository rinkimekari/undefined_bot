use serenity::framework::standard::macros::group;

mod joke;
mod ping;

use joke::JOKE_COMMAND;
use ping::PING_COMMAND;

#[group]
#[commands(ping, joke)]
pub struct General;
