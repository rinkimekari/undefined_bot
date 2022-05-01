use serenity::framework::standard::macros::group;

mod cat;
mod dog;
mod gnu;
mod help;
mod joke;
mod leet;
mod linux;
mod ping;
mod virustotal;
mod ip;
mod url;

use cat::CAT_COMMAND;
use dog::DOG_COMMAND;
use gnu::GNU_COMMAND;
use help::HELP_COMMAND;
use joke::JOKE_COMMAND;
use leet::LEET_COMMAND;
use linux::LINUX_COMMAND;
use ping::PING_COMMAND;
use virustotal::VIRUSTOTAL_COMMAND;
use ip::IP_COMMAND;
use self::url::URL_COMMAND;

#[group]
#[commands(ping, joke, gnu, leet, help, linux, dog, cat, virustotal, ip, url)]
pub struct General;
