use serenity::{
    all::{CommandInteraction, Context, CreateCommand},
    async_trait,
};
use songbird::tracks::TrackHandle;

use crate::commands::{CommandResult, RCommand};

pub struct Skip;

#[async_trait]
impl RCommand for Skip {
    fn name(&self) -> &str {
        "skip"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Salta la canción que está sonando")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        super::control(ctx, cmd, TrackHandle::stop, "⏭️ Saltada").await
    }
}
