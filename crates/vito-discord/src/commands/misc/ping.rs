use std::time::Instant;

use serenity::{
    all::{
        CommandInteraction, Context, CreateCommand, CreateInteractionResponseMessage,
        EditInteractionResponse,
    },
    async_trait,
};

use crate::commands::{CommandResult, VCommand, reply};

pub struct Ping;

#[async_trait]
impl VCommand for Ping {
    fn name(&self) -> &str {
        "ping"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Testea la latencia del bot")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let before = Instant::now();

        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().content("Pong!"),
        )
        .await?;

        let elapsed = before.elapsed().as_millis();

        let edited = EditInteractionResponse::new().content(format!("Pong! ({elapsed}ms)"));
        cmd.edit_response(&ctx.http, edited).await?;

        Ok(())
    }
}
