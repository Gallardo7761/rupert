use rupert_core::logging::{RLogger, RLoggerContext};
use serenity::all::{EventHandler, Ready};
use serenity::async_trait;
use serenity::prelude::Context;

pub struct REventHandler {
    logger: RLogger,
}

impl REventHandler {
    pub fn new() -> Self {
        let logger: RLogger = RLogger::new(RLoggerContext::Discord);
        Self { logger }
    }
}

#[async_trait]
impl EventHandler for REventHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        self.logger
            .info(format!("{} initialized. I'm ready papi.", ready.user.name).as_str());
    }
}
