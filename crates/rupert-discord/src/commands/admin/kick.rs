use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        CreateInteractionResponseMessage, Permissions, ResolvedOption,
    },
    async_trait,
};

use super::moderation;
use crate::commands::{CommandResult, RSubcommand, options, reply};

pub struct Kick;

#[async_trait]
impl RSubcommand for Kick {
    fn name(&self) -> &str {
        "kick"
    }

    fn permission(&self) -> Permissions {
        Permissions::KICK_MEMBERS
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Expulsa a un miembro del servidor",
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "A quién expulsar")
                .required(true),
        )
        .add_sub_option(moderation::reason_option())
    }

    async fn run(
        &self,
        ctx: &Context,
        cmd: &CommandInteraction,
        options: &[ResolvedOption<'_>],
    ) -> CommandResult {
        let Some((user, member)) = options::get_member(options, "user") else {
            return reply::send(ctx, cmd, reply::error_message("Necesito un usuario")).await;
        };
        let reason = options::get_string(options, "reason");

        if let Some(denial) = moderation::member_denial(ctx, cmd, &user, member.as_ref()).await? {
            return reply::send(ctx, cmd, reply::error_message(&denial)).await;
        }

        let audit = moderation::audit_reason(cmd, reason.as_deref());
        moderation::guild_id(cmd)?
            .kick_with_reason(&ctx.http, user.id, &audit)
            .await?;

        let embed = moderation::action_embed("👢 Expulsado", cmd, &user, reason.as_deref());
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
