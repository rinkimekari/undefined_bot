use serenity::framework::standard::macros::group;

mod cat;
mod cunt;
mod dog;
mod github;
mod gnu;
mod help;
mod ip;
mod joke;
mod leet;
mod linux;
mod ping;
mod url;
mod virustotal;

use self::url::URL_COMMAND;
use cat::CAT_COMMAND;
use cunt::CUNT_COMMAND;
use dog::DOG_COMMAND;
use github::GITHUB_COMMAND;
use gnu::GNU_COMMAND;
use help::HELP_COMMAND;
use ip::IP_COMMAND;
use joke::JOKE_COMMAND;
use leet::LEET_COMMAND;
use linux::LINUX_COMMAND;
use ping::PING_COMMAND;
use virustotal::VIRUSTOTAL_COMMAND;

#[group]
#[commands(
    ping, joke, gnu, leet, help, linux, dog, cat, virustotal, ip, url, github, cunt
)]
pub struct General;
