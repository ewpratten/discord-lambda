# Discord bots as Vercel lambda functions

[![Build](https://github.com/Ewpratten/discord-lambda/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/discord-lambda/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/discord-lambda/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/discord-lambda/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/discord-lambda/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/discord-lambda/actions/workflows/audit.yml)

This project aims to create an easy method for building Discord bots on the Vercel platform.

The benefit of building bots this way is, you do not have to pay for unused uptime, since the handlers are only spun up when a message is sent.

## Crates in this repo

 - [`discord-lambda`](./discord-lambda) **[WIP]** [![Crates.io](https://img.shields.io/crates/v/discord-lambda)](https://crates.io/crates/discord-lambda) [![Docs.rs](https://docs.rs/discord-lambda/badge.svg)](https://docs.rs/discord-lambda) 
   - The primary API for this project. Handles Discord magic in the background, while routing command calls to the client code.
 - [`discord-message`](./discord-lambda) [![Crates.io](https://img.shields.io/crates/v/discord-message)](https://crates.io/crates/discord-message) [![Docs.rs](https://docs.rs/discord-message/badge.svg)](https://docs.rs/discord-message) 
   - Type definitions for creating Discord chat messages and exporting them to webhook-ready JSON.
 - [`discord-webhook-client`](./discord-lambda) [![Crates.io](https://img.shields.io/crates/v/discord-webhook-client)](https://crates.io/crates/discord-webhook-client) [![Docs.rs](https://docs.rs/discord-webhook-client/badge.svg)](https://docs.rs/discord-webhook-client) 
   - A small wrapper library that can send messages via webhook to Discord.
 - [`discord-sendmessage`](./discord-lambda) [![Crates.io](https://img.shields.io/crates/v/discord-sendmessage)](https://crates.io/crates/discord-sendmessage)
   - A debug utility for sending discord messages from the commandline