use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        CreateInteractionResponseMessage, EditMember, Mentionable, Permissions, ResolvedOption,
    },
    async_trait,
};

use super::moderation;
use crate::{
    commands::{CommandResult, RSubcommand, options, reply},
    embeds,
};

const MAX_NICK: u16 = 32;

pub struct Nick;

#[async_trait]
impl RSubcommand for Nick {
    fn name(&self) -> &str {
        "nick"
    }

    fn permission(&self) -> Permissions {
        Permissions::MANAGE_NICKNAMES
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Cambia el apodo de un miembro",
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "A quién cambiar el apodo")
                .required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "nickname",
                "Apodo nuevo (vacío para quitarlo)",
            )
            .max_length(MAX_NICK),
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
        let nickname = options::get_string(options, "nickname");

        if let Some(denial) = moderation::member_denial(ctx, cmd, &user, member.as_ref()).await? {
            return reply::send(ctx, cmd, reply::error_message(&denial)).await;
        }

        let audit = moderation::audit_reason(cmd, None);
        // An empty nickname resets it.
        moderation::guild_id(cmd)?
            .edit_member(
                &ctx.http,
                user.id,
                EditMember::new()
                    .nickname(nickname.clone().unwrap_or_default())
                    .audit_log_reason(&audit),
            )
            .await?;

        let description = match nickname {
            Some(nickname) => format!("✏️ Apodo de {} cambiado a **{nickname}**", user.mention()),
            None => format!("✏️ Apodo de {} quitado", user.mention()),
        };
        let embed = embeds::base().description(description);
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
