use serenity::framework::standard::macros::group;

mod joke;
mod ping;
mod gnu;
mod leet;
mod help;
mod linux;
mod dog;
mod cat;

use joke::JOKE_COMMAND;
use ping::PING_COMMAND;
use gnu::GNU_COMMAND;
use leet::LEET_COMMAND;
use help::HELP_COMMAND;
use linux::LINUX_COMMAND;
use dog::DOG_COMMAND;
use cat::CAT_COMMAND;

#[group]
#[commands(ping, joke, gnu, leet, help, linux, dog, cat)]
pub struct General;
