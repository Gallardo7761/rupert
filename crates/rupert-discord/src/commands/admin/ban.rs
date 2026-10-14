use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        CreateInteractionResponseMessage, Permissions, ResolvedOption,
    },
    async_trait,
};

use super::moderation;
use crate::commands::{CommandResult, RSubcommand, options, reply};

pub struct Ban;

#[async_trait]
impl RSubcommand for Ban {
    fn name(&self) -> &str {
        "ban"
    }

    fn permission(&self) -> Permissions {
        Permissions::BAN_MEMBERS
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Banea a un usuario del servidor",
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "A quién banear")
                .required(true),
        )
        .add_sub_option(moderation::reason_option())
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "delete_messages",
                "Borrar sus mensajes recientes (por defecto, ninguno)",
            )
            .add_int_choice("Ninguno", 0)
            .add_int_choice("Último día", 1)
            .add_int_choice("Últimos 7 días", 7),
        )
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
        let delete_days = options::get_integer(options, "delete_messages")
            .and_then(|d| u8::try_from(d).ok())
            .unwrap_or(0)
            .min(7);

        if member.is_some()
            && let Some(denial) =
                moderation::member_denial(ctx, cmd, &user, member.as_ref()).await?
        {
            return reply::send(ctx, cmd, reply::error_message(&denial)).await;
        }

        let audit = moderation::audit_reason(cmd, reason.as_deref());
        moderation::guild_id(cmd)?
            .ban_with_reason(&ctx.http, user.id, delete_days, &audit)
            .await?;

        let embed = moderation::action_embed("🔨 Baneado", cmd, &user, reason.as_deref());
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
