mod handler;

use rupert_core::logging::{RLogger, RLoggerContext};
use serenity::{Client, all::GatewayIntents};
use std::process::exit;

use crate::handler::REventHandler;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let logger = RLogger::new(RLoggerContext::Discord);

    let token = get_token()?;
    logger.info("I've got my token, logging in...");

    let intents = GatewayIntents::empty();
    let mut client = Client::builder(token, intents)
        .event_handler(REventHandler::new())
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
