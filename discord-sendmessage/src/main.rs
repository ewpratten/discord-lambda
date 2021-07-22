use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use discord_message::DiscordMessage;
use discord_webhook_client::send_message;

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("message")
                .takes_value(true)
                .help("Message to send to discord")
                .required(true),
        )
        .arg(
            Arg::with_name("webhook_id")
                .long("webhook-id")
                .short("i")
                .help("ID portion of the webhook")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("webhook_token")
                .long("webhook-token")
                .short("t")
                .help("Token portion of the webhook")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Get data
    let message = matches.value_of("message").unwrap();
    let wh_id = matches.value_of("webhook_id").unwrap();
    let wh_token = matches.value_of("webhook_token").unwrap();

    // Build the message
    let message = DiscordMessage {
        content: message.to_string(),
        ..Default::default()
    };

    // Build the URL
    let url = format!("https://discord.com/api/webhooks/{}/{}", wh_id, wh_token)
        .parse()
        .unwrap();

    // Send the message
    println!("{:?}", send_message(url, &message).await.unwrap());
}
