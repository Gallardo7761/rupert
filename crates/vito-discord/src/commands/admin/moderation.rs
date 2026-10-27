//! Helpers shared by the `/admin` subcommands that act on members.

use std::collections::HashMap;

use vito_core::{
    error::HierarchyError,
    moderation::{self, Actor},
};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommandOption, CreateEmbed, GuildId,
    Mentionable, PartialGuild, PartialMember, Role, RoleId, User, UserId,
};

use crate::{commands::CommandResult, embeds};

/// Discord's limit for audit log reasons.
const MAX_REASON: u16 = 400;

pub fn guild_id(cmd: &CommandInteraction) -> CommandResult<GuildId> {
    cmd.guild_id
        .ok_or_else(|| "/admin used outside a server".into())
}

pub fn reason_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::String,
        "reason",
        "Motivo (sale en el registro de auditoría)",
    )
    .max_length(MAX_REASON)
}

/// The reason sent to the audit log, which would otherwise only show the bot.
pub fn audit_reason(cmd: &CommandInteraction, reason: Option<&str>) -> String {
    format!("{}: {}", cmd.user.name, reason.unwrap_or("sin motivo"))
}

/// The public embed announcing a moderation action.
pub fn action_embed(
    title: &str,
    cmd: &CommandInteraction,
    target: &User,
    reason: Option<&str>,
) -> CreateEmbed {
    let embed = embeds::base()
        .title(title)
        .field("Usuario", target.mention().to_string(), true)
        .field("Moderador", cmd.user.mention().to_string(), true);
    match reason {
        Some(reason) => embed.field("Motivo", reason, false),
        None => embed,
    }
}

/// The executor, the bot and the server data needed to check the hierarchy.
pub struct Hierarchy {
    executor: Actor,
    bot: Actor,
    guild: PartialGuild,
}

impl Hierarchy {
    pub async fn fetch(ctx: &Context, cmd: &CommandInteraction) -> CommandResult<Self> {
        let guild = guild_id(cmd)?.to_partial_guild(&ctx.http).await?;
        let bot_id = ctx.http.get_current_user().await?.id;
        let bot_member = guild.id.member(&ctx.http, bot_id).await?;
        let executor_roles = cmd.member.as_ref().map_or(&[][..], |m| &m.roles);

        Ok(Self {
            executor: actor(&guild, cmd.user.id, executor_roles),
            bot: actor(&guild, bot_id, &bot_member.roles),
            guild,
        })
    }

    pub fn check_member(
        &self,
        target: &User,
        member: &PartialMember,
    ) -> Result<(), HierarchyError> {
        let target = actor(&self.guild, target.id, &member.roles);
        moderation::check_member(&self.executor, &target, &self.bot)
    }

    pub fn check_role(&self, role: &Role) -> Result<(), HierarchyError> {
        moderation::check_role(&self.executor, role.position, &self.bot)
    }
}

fn actor(guild: &PartialGuild, id: UserId, roles: &[RoleId]) -> Actor {
    Actor {
        id: id.get(),
        top_role: top_role(&guild.roles, roles),
        is_owner: guild.owner_id == id,
    }
}

fn top_role(guild_roles: &HashMap<RoleId, Role>, roles: &[RoleId]) -> u16 {
    roles
        .iter()
        .filter_map(|id| guild_roles.get(id))
        .map(|role| role.position)
        .max()
        .unwrap_or(0)
}

/// Checks that the target is in the server and that the hierarchy allows acting on them.
/// Returns the message to show the executor when it does not.
pub async fn member_denial(
    ctx: &Context,
    cmd: &CommandInteraction,
    target: &User,
    member: Option<&PartialMember>,
) -> CommandResult<Option<String>> {
    let Some(member) = member else {
        return Ok(Some("Ese usuario no está en el servidor".to_string()));
    };
    let hierarchy = Hierarchy::fetch(ctx, cmd).await?;
    Ok(hierarchy
        .check_member(target, member)
        .err()
        .map(|e| e.to_string()))
}
