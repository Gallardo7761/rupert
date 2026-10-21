use serenity::{
    all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseMessage},
    async_trait,
};

use super::{guild_id, players, track_embed};
use crate::commands::{CommandResult, RCommand, reply};

const MAX_SHOWN: usize = 10;
const MAX_TITLE: usize = 80;

pub struct Queue;

#[async_trait]
impl RCommand for Queue {
    fn name(&self) -> &str {
        "queue"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Muestra lo que está sonando y lo que viene")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let guild_id = guild_id(cmd)?;

        let embed = {
            let players = players(ctx).await?;
            let players = players.lock().await;
            let queue = players.get(&guild_id).map(|player| &player.queue);
            queue.and_then(|queue| {
                let current = queue.current()?;
                let mut upcoming: Vec<String> = queue
                    .upcoming()
                    .take(MAX_SHOWN)
                    .enumerate()
                    .map(|(i, track)| {
                        let title: String = track.title().chars().take(MAX_TITLE).collect();
                        format!("{}. {title}", i + 1)
                    })
                    .collect();

                let hidden = queue.upcoming().count().saturating_sub(MAX_SHOWN);
                if hidden > 0 {
                    upcoming.push(format!("…y {hidden} más"));
                }

                let embed = track_embed(current).title("🎶 Sonando");
                Some(if upcoming.is_empty() {
                    embed
                } else {
                    embed.field("Siguientes", upcoming.join("\n"), false)
                })
            })
        };

        let Some(embed) = embed else {
            return reply::send(ctx, cmd, reply::error_message("La cola está vacía")).await;
        };
        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
