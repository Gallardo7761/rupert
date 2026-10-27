use serenity::{
    all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseMessage},
    async_trait,
};

use super::{listener_guild, manager, players};
use crate::{
    commands::{CommandResult, VCommand, reply},
    embeds,
};

pub struct Leave;

#[async_trait]
impl VCommand for Leave {
    fn name(&self) -> &str {
        "leave"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Saca al bot del canal de voz")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let Some(guild_id) = listener_guild(ctx, cmd).await? else {
            return Ok(());
        };

        players(ctx).await?.lock().await.remove(&guild_id);
        manager(ctx).await?.remove(guild_id).await?;

        let embed = embeds::base().description("👋 Me piro");
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
