use rupert_core::logging::{RLogger, RLoggerContext};
use serenity::all::{EventHandler, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::prelude::Context;

use crate::commands::{self, RCommand};

pub struct Bot {
    logger: RLogger,
    guild_ids: Vec<GuildId>,
    commands: Vec<Box<dyn RCommand>>,
}

impl Bot {
    pub fn new(guild_ids: Vec<GuildId>) -> Self {
        let logger: RLogger = RLogger::new(RLoggerContext::Discord);
        Self {
            logger,
            guild_ids,
            commands: commands::get_all(),
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        self.logger.info("Registering commands...");

        for id in &self.guild_ids {
            match id
                .set_commands(
                    &ctx.http,
                    self.commands.iter().map(|c| c.register()).collect(),
                )
                .await
            {
                Ok(registered) => {
                    let names = registered
                        .iter()
                        .map(|c| c.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.logger
                        .info(&format!("Registered in guild {id}: {names}"));
                }
                Err(e) => {
                    self.logger
                        .error(&format!("Could not register commands in guild {id}"), &e);
                }
            }
        }

        self.logger
            .info(format!("{} initialized. I'm ready papi.", ready.user.name).as_str());
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            let place = cmd.guild_id.map_or("DM".to_string(), |id| id.to_string());

            self.logger.info(&format!(
                "{} ran /{} on {}",
                cmd.user.name, cmd.data.name, place
            ));

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
