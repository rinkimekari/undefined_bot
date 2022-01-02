use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;


// TODO: use hashmap for proper leetcode approval
// TODO: maybe add more characters for cool stuffs

#[command]
pub async fn leet(ctx: &Context, msg: &Message) -> CommandResult {
    let prefix = format!(
        "{}{}",
        std::env::var("BOT_PREFIX").unwrap(),
        "leet"
    );
    let args: Vec<&str> = msg
                            .content
                            .strip_prefix(&prefix)
                            .unwrap()
                            .split(' ')
                            .collect();
    let mut result = String::new();

    for s in args {
        for c in s.chars() {
            result.push(get_leet(c));
        }
        result.push(' ');
    }

    msg.reply(ctx, result).await?;

    Ok(())
}

fn get_leet(c: char) -> char {
    match c {
        'a' => '4',
        'b' => '8',
        'c' => 'c', 
        'd' => 'd', 
        'e' => '3', 
        'f' => 'f', 
        'g' => 'g', 
        'h' => 'h', 
        'i' => '!', 
        'j' => 'j', 
        'k' => 'k', 
        'l' => '1', 
        'm' => 'm', 
        'n' => 'n', 
        'o' => '0', 
        'p' => 'p', 
        'q' => 'q', 
        'r' => 'r', 
        's' => 's', 
        't' => '7', 
        'u' => 'u', 
        'v' => 'v', 
        'w' => 'w', 
        'x' => '*', 
        'y' => 'y', 
        'z' => 'z', 
        _ => c,
    }
}
