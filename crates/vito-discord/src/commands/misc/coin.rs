use vito_core::fun::flip_coin;
use serenity::{
    all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseMessage},
    async_trait,
};

use crate::{
    commands::{CommandResult, VCommand, reply},
    embeds,
};

pub struct CoinFlip;

#[async_trait]
impl VCommand for CoinFlip {
    fn name(&self) -> &str {
        "coin"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Tira una moneda al aire")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let embed = embeds::base().title("🪙 Se ha lanzado la moneda").field(
            "Resultado",
            flip_coin().to_string(),
            false,
        );

        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
