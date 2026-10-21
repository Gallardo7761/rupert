mod bot;
mod commands;
mod embeds;
mod permissions;

use commands::music::{HttpKey, PlayersKey};
use rupert_core::logging::{RLogger, RLoggerContext};

use serenity::{
    Client,
    all::{GatewayIntents, GuildId},
};
use songbird::SerenityInit;
use std::process::exit;

use crate::bot::Bot;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let logger = RLogger::new(RLoggerContext::Discord);

    let token = get_token()?;
    logger.info("I've got my token, logging in...");

    let guild_ids = get_guild_ids()?;
    logger.info(format!("Chatting on guilds {:?}", guild_ids).as_str());

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES;
    let mut client = Client::builder(token, intents)
        .event_handler(Bot::new(guild_ids))
        .register_songbird()
        .type_map_insert::<HttpKey>(reqwest::Client::new())
        .type_map_insert::<PlayersKey>(Default::default())
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

fn get_guild_ids() -> Result<Vec<GuildId>, Box<dyn std::error::Error>> {
    let raw = dotenvy::var("GUILD_IDS")?;
    let ids = raw
        .split(',')
        .map(|s| s.trim().parse::<u64>().map(GuildId::new))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ids)
}
