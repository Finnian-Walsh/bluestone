mod error;
mod handler;
mod slash_commands;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use handler::Handler;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token = env::var("BLUESTONE_TOKEN").expect("Expected BLUESTONE_TOKEN in environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .context("Err creating client")?;

    client.start().await.context("Failed to start client")?;

    Ok(())
}
