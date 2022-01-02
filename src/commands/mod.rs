use serenity::framework::standard::macros::group;

mod ping;
mod gnu;

use ping::PING_COMMAND;
use gnu::GNU_COMMAND;

#[group]
#[commands(ping, gnu)]
pub struct General;
