use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        CreateInteractionResponseMessage, EditMember, Permissions, ResolvedOption, Timestamp,
    },
    async_trait,
};

use super::moderation;
use crate::commands::{CommandResult, RSubcommand, options, reply};

/// Discord's maximum timeout: 28 days.
const MAX_MINUTES: i64 = 28 * 24 * 60;

pub struct Timeout;

#[async_trait]
impl RSubcommand for Timeout {
    fn name(&self) -> &str {
        "timeout"
    }

    fn permission(&self) -> Permissions {
        Permissions::MODERATE_MEMBERS
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Aísla a un miembro durante un tiempo",
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "A quién aislar")
                .required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::Integer, "duration", "Cuánto tiempo")
                .required(true)
                .add_int_choice("1 minuto", 1)
                .add_int_choice("5 minutos", 5)
                .add_int_choice("10 minutos", 10)
                .add_int_choice("1 hora", 60)
                .add_int_choice("1 día", 24 * 60)
                .add_int_choice("1 semana", 7 * 24 * 60),
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
        let Some(minutes) =
            options::get_integer(options, "duration").filter(|m| (1..=MAX_MINUTES).contains(m))
        else {
            return reply::send(ctx, cmd, reply::error_message("Duración no válida")).await;
        };
        let reason = options::get_string(options, "reason");

        if let Some(denial) = moderation::member_denial(ctx, cmd, &user, member.as_ref()).await? {
            return reply::send(ctx, cmd, reply::error_message(&denial)).await;
        }
        // Discord refuses to time out administrators, whatever their roles.
        if member
            .as_ref()
            .and_then(|m| m.permissions)
            .is_some_and(|p| p.administrator())
        {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("No se puede aislar a un administrador"),
            )
            .await;
        }

        let until =
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + minutes * 60)?;
        let audit = moderation::audit_reason(cmd, reason.as_deref());
        moderation::guild_id(cmd)?
            .edit_member(
                &ctx.http,
                user.id,
                EditMember::new()
                    .disable_communication_until_datetime(until)
                    .audit_log_reason(&audit),
            )
            .await?;

        let embed = moderation::action_embed("🔇 Aislado", cmd, &user, reason.as_deref()).field(
            "Hasta",
            format!("<t:{}:f>", until.unix_timestamp()),
            true,
        );
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
