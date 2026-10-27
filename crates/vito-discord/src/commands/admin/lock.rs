use serenity::{
    all::{
        ChannelId, ChannelType, CommandInteraction, CommandOptionType, Context,
        CreateCommandOption, CreateInteractionResponseMessage, Mentionable, PermissionOverwrite,
        PermissionOverwriteType, Permissions, ResolvedOption,
    },
    async_trait,
};

use super::moderation;
use crate::{
    commands::{CommandResult, VSubcommand, options, reply},
    embeds,
};

/// What `lock` takes away from `@everyone`.
const LOCKED: Permissions = Permissions::SEND_MESSAGES.union(Permissions::SEND_MESSAGES_IN_THREADS);

pub struct Lock;
pub struct Unlock;

#[async_trait]
impl VSubcommand for Lock {
    fn name(&self) -> &str {
        "lock"
    }

    fn permission(&self) -> Permissions {
        Permissions::MANAGE_CHANNELS
    }

    fn bot_permission(&self) -> Permissions {
        Permissions::MANAGE_ROLES
    }

    fn register(&self) -> CreateCommandOption {
        register(self.name(), "Impide que @everyone escriba en un canal")
    }

    async fn run(
        &self,
        ctx: &Context,
        cmd: &CommandInteraction,
        options: &[ResolvedOption<'_>],
    ) -> CommandResult {
        set_locked(ctx, cmd, options, true).await
    }
}

#[async_trait]
impl VSubcommand for Unlock {
    fn name(&self) -> &str {
        "unlock"
    }

    fn permission(&self) -> Permissions {
        Permissions::MANAGE_CHANNELS
    }

    fn bot_permission(&self) -> Permissions {
        Permissions::MANAGE_ROLES
    }

    fn register(&self) -> CreateCommandOption {
        register(self.name(), "Quita el bloqueo de un canal")
    }

    async fn run(
        &self,
        ctx: &Context,
        cmd: &CommandInteraction,
        options: &[ResolvedOption<'_>],
    ) -> CommandResult {
        set_locked(ctx, cmd, options, false).await
    }
}

fn register(name: &str, description: &str) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, name, description).add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::Channel,
            "channel",
            "Qué canal (por defecto, este)",
        )
        .channel_types(vec![ChannelType::Text, ChannelType::News]),
    )
}

/// Edits the `@everyone` overwrite of the channel.
async fn set_locked(
    ctx: &Context,
    cmd: &CommandInteraction,
    options: &[ResolvedOption<'_>],
    locked: bool,
) -> CommandResult {
    let channel_id: ChannelId = options::get_channel(options, "channel").unwrap_or(cmd.channel_id);
    let everyone = moderation::guild_id(cmd)?.everyone_role();

    let Some(channel) = channel_id.to_channel(&ctx.http).await?.guild() else {
        return reply::send(
            ctx,
            cmd,
            reply::error_message("Ese canal no es de un servidor"),
        )
        .await;
    };
    let kind = PermissionOverwriteType::Role(everyone);
    let (mut allow, mut deny) = channel
        .permission_overwrites
        .iter()
        .find(|o| o.kind == kind)
        .map_or((Permissions::empty(), Permissions::empty()), |o| {
            (o.allow, o.deny)
        });

    if deny.contains(LOCKED) == locked {
        let text = if locked {
            "Ese canal ya está bloqueado"
        } else {
            "Ese canal no está bloqueado"
        };
        return reply::send(ctx, cmd, reply::error_message(text)).await;
    }

    if locked {
        allow.remove(LOCKED);
        deny.insert(LOCKED);
    } else {
        deny.remove(LOCKED);
    }

    if allow.is_empty() && deny.is_empty() {
        channel_id.delete_permission(&ctx.http, kind).await?;
    } else {
        channel_id
            .create_permission(&ctx.http, PermissionOverwrite { allow, deny, kind })
            .await?;
    }

    let description = if locked {
        format!("🔒 {} bloqueado", channel_id.mention())
    } else {
        format!("🔓 {} desbloqueado", channel_id.mention())
    };
    let embed = embeds::base().description(description);
    reply::send(
        ctx,
        cmd,
        CreateInteractionResponseMessage::new().embed(embed),
    )
    .await
}
