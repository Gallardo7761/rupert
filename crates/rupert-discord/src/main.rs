mod bot;
mod commands;

use rupert_core::logging::{RLogger, RLoggerContext};
use serenity::{
    Client,
    all::{GatewayIntents, GuildId},
};
use std::process::exit;

use crate::bot::Bot;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let logger = RLogger::new(RLoggerContext::Discord);

    let token = get_token()?;
    logger.info("I've got my token, logging in...");

    let guild_id = get_guild_id()?;
    logger.info(format!("Chatting on guild {}", guild_id).as_str());

    let intents = GatewayIntents::empty();
    let mut client = Client::builder(token, intents)
        .event_handler(Bot::new(guild_id))
        .await?;

    client.start().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let logger = RLogger::new(RLoggerContext::Discord);

    if let Err(e) = run().await {
        logger.error("Shutting down...", e.as_ref());
        exit(1);
    }
}

fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    Ok(dotenvy::var("DISCORD_TOKEN")?)
}

fn get_guild_id() -> Result<GuildId, Box<dyn std::error::Error>> {
    let guild_id = dotenvy::var("GUILD_ID")?.parse::<u64>()?;
    Ok(GuildId::new(guild_id))
}
