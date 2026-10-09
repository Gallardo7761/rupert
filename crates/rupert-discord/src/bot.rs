use rupert_core::logging::{RLogger, RLoggerContext};
use serenity::all::{EventHandler, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::prelude::Context;

use crate::commands::{self, RCommand};

pub struct Bot {
    logger: RLogger,
    guild_id: GuildId,
    commands: Vec<Box<dyn RCommand>>,
}

impl Bot {
    pub fn new(guild_id: GuildId) -> Self {
        let logger: RLogger = RLogger::new(RLoggerContext::Discord);
        Self {
            logger,
            guild_id,
            commands: commands::get_all(),
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        self.logger.info("Registering commands...");

        if let Err(e) = self
            .guild_id
            .set_commands(
                &ctx.http,
                self.commands.iter().map(|c| c.register()).collect(),
            )
            .await
        {
            self.logger.error(format!("Could not register command").as_str(), &e);
        } else {
            self.logger.info("Command registered");
        }

        self.logger
            .info(format!("{} initialized. I'm ready papi.", ready.user.name).as_str());
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            self.logger
                .info(&format!("{} ran /{}", cmd.user.name, cmd.data.name));

            let Some(command) = self
                .commands
                .iter()
                .find(|c| c.name() == cmd.data.name.as_str())
            else {
                self.logger
                    .warn(&format!("Unknown command /{}", cmd.data.name));
                return;
            };

            if let Err(e) = command.run(&ctx, &cmd).await {
                self.logger.error("Error running command", e.as_ref());
            }
        }
    }
}
