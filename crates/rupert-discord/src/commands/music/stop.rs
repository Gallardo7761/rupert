use serenity::{
    all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseMessage},
    async_trait,
};

use super::{listener_guild, players};
use crate::{
    commands::{CommandResult, RCommand, reply},
    embeds,
};

pub struct Stop;

#[async_trait]
impl RCommand for Stop {
    fn name(&self) -> &str {
        "stop"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Para la música y vacía la cola")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let Some(guild_id) = listener_guild(ctx, cmd).await? else {
            return Ok(());
        };

        let handle = {
            let players = players(ctx).await?;
            let mut players = players.lock().await;
            players.get_mut(&guild_id).and_then(|player| {
                player.queue.clear();
                player.handle.take()
            })
        };
        if let Some(handle) = handle {
            handle.stop()?;
        }

        let embed = embeds::base().description("⏹️ Música parada y cola vacía");
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
