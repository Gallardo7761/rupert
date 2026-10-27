use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        CreateInteractionResponseMessage, Mentionable, Permissions, ResolvedOption,
    },
    async_trait,
};

use super::moderation::{self, Hierarchy};
use crate::{
    commands::{CommandResult, VSubcommand, options, reply},
    embeds,
};

pub struct Role;

#[async_trait]
impl VSubcommand for Role {
    fn name(&self) -> &str {
        "role"
    }

    fn permission(&self) -> Permissions {
        Permissions::MANAGE_ROLES
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Da o quita un rol a un miembro",
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::String, "action", "Qué hacer")
                .required(true)
                .add_string_choice("Dar", "add")
                .add_string_choice("Quitar", "remove"),
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "A quién").required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::Role, "role", "Qué rol").required(true),
        )
    }

    async fn run(
        &self,
        ctx: &Context,
        cmd: &CommandInteraction,
        options: &[ResolvedOption<'_>],
    ) -> CommandResult {
        let add = match options::get_string(options, "action").as_deref() {
            Some("add") => true,
            Some("remove") => false,
            _ => return reply::send(ctx, cmd, reply::error_message("Acción no válida")).await,
        };
        let (Some((user, Some(member))), Some(role)) = (
            options::get_member(options, "user"),
            options::get_role(options, "role"),
        ) else {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("Necesito un miembro del servidor y un rol"),
            )
            .await;
        };
        let guild_id = moderation::guild_id(cmd)?;

        // @everyone shares its id with the server; managed roles belong to bots and boosts.
        if role.id.get() == guild_id.get() || role.managed {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("Ese rol no se puede dar ni quitar a mano"),
            )
            .await;
        }
        if member.roles.contains(&role.id) == add {
            let text = if add {
                "Ya tiene ese rol"
            } else {
                "No tiene ese rol"
            };
            return reply::send(ctx, cmd, reply::error_message(text)).await;
        }

        let hierarchy = Hierarchy::fetch(ctx, cmd).await?;
        if let Err(e) = hierarchy
            .check_member(&user, &member)
            .and_then(|()| hierarchy.check_role(&role))
        {
            return reply::send(ctx, cmd, reply::error_message(&e.to_string())).await;
        }

        let audit = moderation::audit_reason(cmd, None);
        let description = if add {
            ctx.http
                .add_member_role(guild_id, user.id, role.id, Some(&audit))
                .await?;
            format!(
                "➕ {} ahora tiene el rol {}",
                user.mention(),
                role.mention()
            )
        } else {
            ctx.http
                .remove_member_role(guild_id, user.id, role.id, Some(&audit))
                .await?;
            format!(
                "➖ {} ya no tiene el rol {}",
                user.mention(),
                role.mention()
            )
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
