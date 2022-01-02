use serenity::framework::standard::macros::group;

mod joke;
mod ping;
mod gnu;

use joke::JOKE_COMMAND;
use ping::PING_COMMAND;
use gnu::GNU_COMMAND;

#[group]
#[commands(ping, joke, gnu)]
pub struct General;
