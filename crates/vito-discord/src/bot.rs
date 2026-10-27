use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use vito_core::logging::{VLogger, VLoggerContext};
use vito_core::status::VActivityType::{Listening, Playing, Watching};
use vito_core::status::{self, VStatus};
use serenity::all::{
    ActivityData, CommandInteraction, CreateCommand, EventHandler, GuildId, Interaction, Ready,
};
use serenity::async_trait;
use serenity::prelude::Context;

use crate::commands::{self, VCommand, reply};
use crate::permissions::has_permission;

const STATUS_INTERVAL: Duration = Duration::from_mins(30);

pub struct Bot {
    logger: VLogger,
    guild_ids: Vec<GuildId>,
    commands: Vec<Box<dyn VCommand>>,
    status_started: AtomicBool,
}

impl Bot {
    pub fn new(guild_ids: Vec<GuildId>) -> Self {
        let logger: VLogger = VLogger::new(VLoggerContext::Discord);
        Self {
            logger,
            guild_ids,
            commands: commands::get_all(),
            status_started: AtomicBool::new(false),
        }
    }

    fn command(&self, name: &str) -> Option<&dyn VCommand> {
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

    fn start_status_rotation(&self, ctx: Context) {
        if self.status_started.swap(true, Ordering::Relaxed) {
            return;
        }

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(STATUS_INTERVAL);
            loop {
                interval.tick().await;
                ctx.set_activity(Some(activity(status::random())));
            }
        });
        self.logger.info("Status rotation started");
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

        self.start_status_rotation(ctx);

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

fn build_command(command: &dyn VCommand) -> CreateCommand {
    let builder = command.register();
    match command.permission() {
        Some(permissions) => builder.default_member_permissions(permissions),
        None => builder,
    }
}

fn activity(status: VStatus) -> ActivityData {
    match status.activity_type() {
        Playing => ActivityData::playing(status.name()),
        Listening => ActivityData::listening(status.name()),
        Watching => ActivityData::watching(status.name()),
    }
}
