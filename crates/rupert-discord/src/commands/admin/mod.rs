pub mod ban;
pub mod clear;
pub mod kick;
pub mod lock;
mod moderation;
pub mod nick;
pub mod poll;
pub mod role;
pub mod say;
pub mod timeout;
pub mod unban;
pub mod untimeout;

use serenity::{
    all::{CommandInteraction, Context, CreateCommand, Permissions, ResolvedOption, ResolvedValue},
    async_trait,
};

use crate::{
    commands::{CommandResult, RCommand, RSubcommand, reply},
    permissions::has_permission,
};

pub struct Admin {
    subcommands: Vec<Box<dyn RSubcommand>>,
}

impl Admin {
    pub fn new() -> Self {
        Self {
            subcommands: vec![
                Box::new(clear::Clear),
                Box::new(kick::Kick),
                Box::new(ban::Ban),
                Box::new(unban::Unban),
                Box::new(timeout::Timeout),
                Box::new(untimeout::Untimeout),
                Box::new(nick::Nick),
                Box::new(role::Role),
                Box::new(lock::Lock),
                Box::new(lock::Unlock),
            ],
        }
    }
}

#[async_trait]
impl RCommand for Admin {
    fn name(&self) -> &str {
        "admin"
    }

    fn permission(&self) -> Option<Permissions> {
        Some(Permissions::MODERATE_MEMBERS)
    }

    fn register(&self) -> CreateCommand {
        self.subcommands.iter().fold(
            CreateCommand::new(self.name()).description("Herramientas de moderación"),
            |command, sub| command.add_option(sub.register()),
        )
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let options = cmd.data.options();

        let Some(ResolvedOption {
            name,
            value: ResolvedValue::SubCommand(inner),
            ..
        }) = options.first()
        else {
            return reply::send(ctx, cmd, reply::error_message("Elige un subcomando")).await;
        };

        let Some(sub) = self.subcommands.iter().find(|s| s.name() == *name) else {
            return reply::send(ctx, cmd, reply::error_message("Subcomando desconocido")).await;
        };

        if !has_permission(cmd, sub.permission()) {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("No tienes permisos para usar este subcomando"),
            )
            .await;
        }

        let bot_permission = sub.bot_permission();
        if !cmd
            .app_permissions
            .is_some_and(|p| p.contains(bot_permission))
        {
            let missing = bot_permission - cmd.app_permissions.unwrap_or_else(Permissions::empty);
            let names = missing.get_permission_names().join(", ");
            return reply::send(
                ctx,
                cmd,
                reply::error_message(&format!("Me faltan permisos en este canal: {names}")),
            )
            .await;
        }

        sub.run(ctx, cmd, inner).await
    }
}
