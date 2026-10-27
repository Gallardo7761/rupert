use serenity::{
    all::{CommandInteraction, Context, CreateCommand},
    async_trait,
};
use songbird::tracks::TrackHandle;

use crate::commands::{CommandResult, VCommand};

pub struct Pause;

#[async_trait]
impl VCommand for Pause {
    fn name(&self) -> &str {
        "pause"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Pausa la canción que está sonando")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        super::control(ctx, cmd, TrackHandle::pause, "⏸️ Pausada").await
    }
}
