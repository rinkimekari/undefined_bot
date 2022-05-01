use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use std::net::Ipv4Addr;
use std::str::FromStr;
use vt3::VtClient;

#[command]
pub async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
    let api_key = std::env::var("VIRUSTOTAL_KEY").unwrap_or_else(|_| "".to_string());
    if api_key.is_empty() {
        msg.reply(ctx, "No API key found \\:(").await?;
        return Ok(());
    }

    let args: Vec<&str> = msg.content.split(' ').collect();

    let ip = *args.get(1).unwrap_or(&"");
    if Ipv4Addr::from_str(ip).is_err() {
        msg.reply(
            ctx,
            "Please provide an IPv4 address so I can actually do something.",
        )
        .await?;
        return Ok(());
    }

    let client = VtClient::new(&api_key);

    let info = ip_info(&client, ip);

    msg.reply(ctx, info).await?;

    Ok(())
}

fn ip_info(client: &VtClient, ip: &str) -> String {
    let link = format!("Link to analysis: <https://www.virustotal.com/gui/ip-address/{ip}>");

    match client.ip_info(ip) {
        Ok(i) => {
            if let Some(a) = i.data.attributes {
                if let Some(s) = a.last_analysis_stats {
                    let mut elements = vec![
                        (s.harmless.unwrap(), "harmless"),
                        (s.malicious.unwrap(), "malicious"),
                        (s.suspicious.unwrap(), "suspicious"),
                    ];

                    elements.sort_by(|a, b| b.0.cmp(&a.0));

                    if elements[0].0 == 0 {
                        "No data found for {ip}".to_string()
                    } else if elements[1].0 == 0 {
                        format!(
                            "Most likely {} ({} hits). {link}",
                            elements[0].1, elements[0].0
                        )
                    } else {
                        format!(
                            "Most likely {} ({} hits), but may be {} ({} hits). {link}",
                            elements[0].1, elements[0].0, elements[1].1, elements[1].0
                        )
                    }
                } else {
                    "Analysis stats not found.".to_string()
                }
            } else {
                "Attributes not found.".to_string()
            }
        }
        Err(e) => e.to_string(),
    }
}
