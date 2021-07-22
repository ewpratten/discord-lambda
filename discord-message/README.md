# discord-message
[![Crates.io](https://img.shields.io/crates/v/discord-message)](https://crates.io/crates/discord-message) 
[![Docs.rs](https://docs.rs/discord-message/badge.svg)](https://docs.rs/discord-message) 


`discord-message` is a crate containing the utilities needed to build Discord webhook messages from Rust

## Example message creation

```rust
fn main() {
    let message = DiscordMessage {
        username: Some("BotMan".to_string()),
        content: "Text message. Up to 2000 characters.".to_string(),
        embeds: vec![
            Embed {
                title: "Title".to_string(),
                description: "Text message. You can use Markdown here.".to_string(),
                ..Default::default()
            }
        ],
        ..Default::default()
    };
    let json = message.to_json().unwrap();
}
```
