use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        CreateInteractionResponseMessage, Permissions, ResolvedOption,
    },
    async_trait,
};

use super::moderation;
use crate::commands::{CommandResult, VSubcommand, options, reply};

pub struct Unban;

#[async_trait]
impl VSubcommand for Unban {
    fn name(&self) -> &str {
        "unban"
    }

    fn permission(&self) -> Permissions {
        Permissions::BAN_MEMBERS
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Quita el baneo a un usuario",
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "A quién desbanear (puedes pegar su ID)",
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
        // A banned user is not a member, so only the user matters here.
        let Some(user) = options::get_user(options, "user") else {
            return reply::send(ctx, cmd, reply::error_message("Necesito un usuario")).await;
        };
        let reason = options::get_string(options, "reason");
        let guild_id = moderation::guild_id(cmd)?;

        if ctx.http.get_ban(guild_id, user.id).await?.is_none() {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("Ese usuario no está baneado"),
            )
            .await;
        }

        let audit = moderation::audit_reason(cmd, reason.as_deref());
        ctx.http.remove_ban(guild_id, user.id, Some(&audit)).await?;

        let embed = moderation::action_embed("🕊️ Desbaneado", cmd, &user, reason.as_deref());
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
