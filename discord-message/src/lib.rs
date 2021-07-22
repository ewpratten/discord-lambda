//! `discord-message` is a crate containing the utilities needed to build Discord webhook messages from Rust.
//!
//! ## Example message creation
//!
//! ```
//! fn main() {
//!     let message = DiscordMessage {
//!         username: Some("BotMan".to_string()),
//!         content: "Text message. Up to 2000 characters.".to_string(),
//!         embeds: vec![
//!             Embed {
//!                 title: "Title".to_string(),
//!                 description: "Text message. You can use Markdown here.".to_string(),
//!                 ..Default::default()
//!             }
//!         ],
//!         ..Default::default()
//!     };
//!     let json = message.to_json().unwrap();
//! }
//! ```

use serde::{Deserialize, Serialize};
use url::Url;

/// Describes a field that can be used inside a message embed
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbedField {
    /// Field title
    #[serde(rename = "name")]
    pub title: String,
    /// Field value
    pub value: String,
    /// If true, the field will be displayed on the same line as the last
    pub inline: bool,
}

/// Describes an embed author
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbedAuthor {
    /// Author name
    pub name: String,
    /// URL of the author
    pub url: Option<Url>,
    /// Avatar URL for the author
    pub icon_url: Option<Url>,
}

/// Describes an embed thumbnail
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    /// Thumbnail URL
    pub url: Url,
}

/// Describes an embed image
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImage {
    /// Image URL
    pub url: Url,
}

/// Describes an embed footer
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbedFooter {
    /// Footer text
    pub text: String,
    /// Footer icon URL
    pub icon_url: Option<Url>,
}

/// Describes an embed
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Embed {
    /// The title of the embed
    pub title: String,
    /// The description of the embed
    pub description: String,
    /// The color of the embed
    pub color: Option<u32>,
    /// The embed author
    pub author: Option<EmbedAuthor>,
    /// Possible fields
    pub fields: Option<Vec<EmbedField>>,
    /// The thumbnail of the embed
    pub thumbnail: Option<EmbedThumbnail>,
    /// The image of the embed
    pub image: Option<EmbedImage>,
    /// The footer of the embed
    pub footer: Option<EmbedFooter>,
}

/// The main message type
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiscordMessage {
    /// Bot username override
    pub username: Option<String>,
    /// Bot avatar override
    pub avatar_url: Option<Url>,
    /// Message content
    pub content: String,
    /// Any possible embeds
    pub embeds: Vec<Embed>,
}

impl DiscordMessage {
    /// Generate JSON data
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
