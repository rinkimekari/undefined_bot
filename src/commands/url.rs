use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use url::Url as ParseUrl;
use vt3::VtClient;

#[command]
pub async fn url(ctx: &Context, msg: &Message) -> CommandResult {
    let api_key = std::env::var("VIRUSTOTAL_KEY").unwrap_or_else(|_| "".to_string());
    if api_key.is_empty() {
        msg.reply(ctx, "No API key found \\:(").await?;
        return Ok(());
    }

    let args: Vec<&str> = msg.content.split(' ').collect();

    let url = *args.get(1).unwrap_or(&"");
    if ParseUrl::parse(url).is_err() {
        msg.reply(ctx, "Please provide a URL so I can actually do something.")
            .await?;
        return Ok(());
    }

    let client = VtClient::new(&api_key);

    let info = url_info(&client, url);

    msg.reply(ctx, info).await?;

    Ok(())
}

fn url_info(client: &VtClient, u: &str) -> String {
    let link = "Link to analysis: Not available, but will eventually support analyses for domains."
        .to_string();

    match client.url_info(u) {
        Ok(i) => {
            if let Some(d) = i.data {
                if let Some(a) = d.attributes {
                    if let Some(s) = a.last_analysis_stats {
                        let mut elements = vec![
                            (s.harmless.unwrap(), "harmless"),
                            (s.malicious.unwrap(), "malicious"),
                            (s.suspicious.unwrap(), "suspicious"),
                        ];

                        elements.sort_by(|a, b| b.0.cmp(&a.0));

                        if elements[0].0 == 0 {
                            "No data found for {u}".to_string()
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
                    "Data not found.".to_string()
                }
            } else {
                "Attributes not found.".to_string()
            }
        }
        Err(e) => e.to_string(),
    }
}
