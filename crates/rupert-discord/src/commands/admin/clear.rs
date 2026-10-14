use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommandOption,
        EditInteractionResponse, GetMessages, Permissions, ResolvedOption, Timestamp,
    },
    async_trait,
};

use crate::{
    commands::{CommandResult, RSubcommand, options},
    embeds,
};

const MAX_AMOUNT: u8 = 100;
const MAX_AGE_SECS: i64 = 14 * 24 * 60 * 60;

pub struct Clear;

#[async_trait]
impl RSubcommand for Clear {
    fn name(&self) -> &str {
        "clear"
    }

    fn permission(&self) -> Permissions {
        Permissions::MANAGE_MESSAGES
    }

    fn bot_permission(&self) -> Permissions {
        Permissions::MANAGE_MESSAGES | Permissions::READ_MESSAGE_HISTORY
    }

    fn register(&self) -> CreateCommandOption {
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            self.name(),
            "Borra los últimos mensajes del canal",
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::Integer, "amount", "Cuántos mensajes")
                .required(true)
                .min_int_value(1)
                .max_int_value(MAX_AMOUNT.into()),
        )
    }

    async fn run(
        &self,
        ctx: &Context,
        cmd: &CommandInteraction,
        options: &[ResolvedOption<'_>],
    ) -> CommandResult {
        let amount = options::get_integer(options, "amount")
            .and_then(|n| u8::try_from(n).ok())
            .unwrap_or(0)
            .min(MAX_AMOUNT);

        cmd.defer_ephemeral(&ctx.http).await?;

        let cutoff = Timestamp::now().unix_timestamp() - MAX_AGE_SECS;
        let ids: Vec<_> = cmd
            .channel_id
            .messages(&ctx.http, GetMessages::new().limit(amount))
            .await?
            .into_iter()
            .filter(|m| !m.pinned && m.timestamp.unix_timestamp() > cutoff)
            .map(|m| m.id)
            .collect();

        let description = if ids.is_empty() {
            "❌ No hay mensajes que pueda borrar (ni fijados ni de hace más de 14 días)".to_string()
        } else {
            cmd.channel_id.delete_messages(&ctx.http, &ids).await?;
            format!("🧹 He borrado **{}** mensajes", ids.len())
        };

        cmd.edit_response(
            &ctx.http,
            EditInteractionResponse::new().embed(embeds::base().description(description)),
        )
        .await?;
        Ok(())
    }
}
