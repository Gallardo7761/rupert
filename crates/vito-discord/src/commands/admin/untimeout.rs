use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        CreateInteractionResponseMessage, EditMember, Permissions, ResolvedOption,
    },
    async_trait,
};

use super::moderation;
use crate::commands::{CommandResult, VSubcommand, options, reply};

pub struct Untimeout;

#[async_trait]
impl VSubcommand for Untimeout {
    fn name(&self) -> &str {
        "untimeout"
    }

    fn permission(&self) -> Permissions {
        Permissions::MODERATE_MEMBERS
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Quita el aislamiento a un miembro",
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "A quién quitar el aislamiento",
            )
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
            .edit_member(
                &ctx.http,
                user.id,
                EditMember::new()
                    .enable_communication()
                    .audit_log_reason(&audit),
            )
            .await?;

        let embed =
            moderation::action_embed("🔊 Aislamiento retirado", cmd, &user, reason.as_deref());
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
