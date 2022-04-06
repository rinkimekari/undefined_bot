use std::net::Ipv4Addr;
use std::str::FromStr;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use vt3::VtClient;

#[command]
pub async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
    let api_key = std::env::var("VIRUSTOTAL_KEY").unwrap_or("".to_string());
    if api_key == "" {
        msg.reply(ctx, "No API key found \\:(").await?;
        return Ok(());
    }

    let args: Vec<&str> = msg.content.split(' ').collect();

    let ip = *args.get(1).unwrap_or(&"");
    if let Err(_) = Ipv4Addr::from_str(ip) {
        msg.reply(ctx, "Please provide an IPv4 address so I can actually do something.").await?;
        return Ok(());
    }

    let client = VtClient::new(&api_key);

    let info = ip_info(&client, ip);

    msg.reply(ctx, info).await?;

    Ok(())
}

fn ip_info(client: &VtClient, ip: &str) -> String {
    match client.ip_info(ip) {
        Ok(i) => {
            let link = if let Some(l) = i.data.links {
                format!("Link to analysis: {}", l.self_field.unwrap())
            } else {
                "Link to analysis not found. That's...not right.".to_owned()
            };

            if let Some(a) = i.data.attributes {
                if let Some(s) = a.last_analysis_stats {
                    let mut elements = Vec::new();
                    elements.push((s.harmless.unwrap(), "harmless"));
                    elements.push((s.malicious.unwrap(), "malicious"));
                    elements.push((s.suspicious.unwrap(), "suspicious"));
                    // elements.push((s.timeout.unwrap(), "timeout"));
                    // elements.push((s.undetected.unwrap(), "undetected"));

                    elements.sort_by(|a, b| b.0.cmp(&a.0));

                    if elements[0].0 == 0 {
                        "No data found for {ip}".to_owned()
                    } else if elements[1].0 == 0 {
                        format!("Most likely {} ({} hits). {}",
                                elements[0].1,
                                elements[0].0,
                                link)
                    } else {
                        format!("Most likely {} ({} hits), but may be {} ({} hits). {}",
                                elements[0].1,
                                elements[0].0,
                                elements[1].1,
                                elements[1].0,
                                link)
                    }
                } else {
                    "Analysis stats not found.".to_owned()
                }
            } else {
                "Attributes not found.".to_owned()
            }
        }
        Err(e) => format!("Error: ```{}```", e.to_string()),
    }
}
