use serenity::framework::standard::macros::group;

mod joke;
mod ping;
mod gnu;
mod leet;

use joke::JOKE_COMMAND;
use ping::PING_COMMAND;
use gnu::GNU_COMMAND;
use leet::LEET_COMMAND;

#[group]
#[commands(ping, joke, gnu, leet)]
pub struct General;
