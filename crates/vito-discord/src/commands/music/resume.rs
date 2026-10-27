use serenity::{
    all::{CommandInteraction, Context, CreateCommand},
    async_trait,
};
use songbird::tracks::TrackHandle;

use crate::commands::{CommandResult, VCommand};

pub struct Resume;

#[async_trait]
impl VCommand for Resume {
    fn name(&self) -> &str {
        "resume"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Sigue con la canción pausada")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        super::control(ctx, cmd, TrackHandle::play, "▶️ Sigue sonando").await
    }
}
