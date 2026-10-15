use rupert_core::logging::{RLogger, RLoggerContext};
use serenity::all::{CommandInteraction, CreateCommand, EventHandler, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::prelude::Context;

use crate::commands::{self, RCommand, reply};
use crate::permissions::has_permission;

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

    fn command(&self, name: &str) -> Option<&dyn RCommand> {
        let command = self.commands.iter().find(|c| c.name() == name);
        if command.is_none() {
            self.logger.warn(&format!("Unknown command /{name}"));
        }
        command.map(|c| c.as_ref())
    }

    async fn run_command(&self, ctx: &Context, cmd: &CommandInteraction) {
        let place = cmd.guild_id.map_or("DM".to_string(), |id| id.to_string());
        self.logger.info(&format!(
            "{} ran /{} on {}",
            cmd.user.name, cmd.data.name, place
        ));

        let Some(command) = self.command(&cmd.data.name) else {
            return;
        };

        if let Some(required) = command.permission()
            && !has_permission(cmd, required)
        {
            let message = reply::error_message("No tienes permisos para usar este comando");
            if let Err(e) = reply::send(ctx, cmd, message).await {
                self.logger.error("Error replying to command", e.as_ref());
            }
            return;
        }

        if let Err(e) = command.run(ctx, cmd).await {
            self.logger.error("Error running command", e.as_ref());
        }
    }

    async fn autocomplete(&self, ctx: &Context, cmd: &CommandInteraction) {
        let Some(command) = self.command(&cmd.data.name) else {
            return;
        };

        if let Err(e) = command.autocomplete(ctx, cmd).await {
            self.logger
                .error("Error autocompleting command", e.as_ref());
        }
    }
}

fn build_command(command: &dyn RCommand) -> CreateCommand {
    let builder = command.register();
    match command.permission() {
        Some(permissions) => builder.default_member_permissions(permissions),
        None => builder,
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
                    self.commands
                        .iter()
                        .map(|c| build_command(c.as_ref()))
                        .collect(),
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
        match interaction {
            Interaction::Command(cmd) => self.run_command(&ctx, &cmd).await,
            Interaction::Autocomplete(cmd) => self.autocomplete(&ctx, &cmd).await,
            _ => {}
        }
    }
}
