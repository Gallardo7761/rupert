use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::embeds;

use crate::commands::CommandResult;

pub async fn send(
    ctx: &Context,
    cmd: &CommandInteraction,
    message: CreateInteractionResponseMessage,
) -> CommandResult {
    cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await?;
    Ok(())
}

pub fn error_message(text: &str) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .embed(embeds::base().description(format!("❌ {text}")))
        .ephemeral(true)
}
