use std::time::Instant;

use serenity::{
    all::{
        CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
    }, async_trait,
};

use crate::commands::{CommandResult, RCommand};

pub struct Ping;

#[async_trait]
impl RCommand for Ping {
    fn name(&self) -> &str {
        "ping"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Testea la latencia del bot")
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let before = Instant::now();
        
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Pong!"),
        );

        cmd.create_response(&ctx.http, response).await?;

        let elapsed = before.elapsed().as_millis();

        let edited = EditInteractionResponse::new()
            .content(format!("Pong! ({}ms)", elapsed));

        cmd.edit_response(&ctx.http, edited).await?;

        Ok(())
    }
}
