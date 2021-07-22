//! `discord-webhook-client` is a super simple client to the Discord webhook service.
//!
//! This crate has one function, `send_message`, which should be used to send messages

use discord_message::DiscordMessage;
use reqwest::Response;

/// Defines possible errors from this crate
#[derive(Debug)]
pub enum Error {
    /// An error with message serialization
    MessageSerializationError(discord_message::Error),

    /// An error with the request
    RequestError(reqwest::Error),
}

impl From<discord_message::Error> for Error {
    fn from(e: discord_message::Error) -> Error {
        Self::MessageSerializationError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Self::RequestError(e)
    }
}

/// Send a message to a discord server via webhook
pub async fn send_message(webhook: url::Url, message: &DiscordMessage) -> Result<Response, Error> {
    let client = reqwest::Client::new();

    // Set up and send the post request
    Ok(client
        .post(webhook)
        .header("Content-Type", "application/json")
        .header(
            "User-Agent",
            "discord-webhook-client/0.x Rust client for Discord webhooks",
        )
        .body(message.to_json()?)
        .send()
        .await?)
}
