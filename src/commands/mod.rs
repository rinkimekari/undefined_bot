use serenity::framework::standard::macros::group;

mod cat;
mod dog;
mod gnu;
mod help;
mod joke;
mod leet;
mod linux;
mod ping;

use cat::CAT_COMMAND;
use dog::DOG_COMMAND;
use gnu::GNU_COMMAND;
use help::HELP_COMMAND;
use joke::JOKE_COMMAND;
use leet::LEET_COMMAND;
use linux::LINUX_COMMAND;
use ping::PING_COMMAND;

#[group]
#[commands(ping, joke, gnu, leet, help, linux, dog, cat)]
pub struct General;
