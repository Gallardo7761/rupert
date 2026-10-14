use rupert_core::fun::banana_size;
use serenity::{
    all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseMessage},
    async_trait,
};

use crate::{
    commands::{CommandResult, RCommand, reply},
    embeds,
};

pub struct Banana;

#[async_trait]
impl RCommand for Banana {
    fn name(&self) -> &str {
        "banana"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Conoce la medida de tu plátano")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let embed = embeds::base().title(format!(
            "El plátano de {} mide {} cm 😳",
            cmd.user.name,
            banana_size()
        ));

        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
