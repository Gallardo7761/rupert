use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, 
        CreateCommand, CreateCommandOption, CreateInteractionResponse, 
        CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
    }, async_trait,
};

use rupert_core::fun;
use crate::embeds;

use crate::commands::{CommandResult, RCommand};

pub struct Dice;

#[async_trait]
impl RCommand for Dice {
    fn name(&self) -> &str {
        "dado"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Tira un dado")
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "tipo",
                "Tipo de dado. Ej: d6, d4, d10"
            ))
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let options = cmd.data.options();
        
        let dice_type = match options.first() {
            Some(ResolvedOption {
                value: ResolvedValue::String(s),
                ..
            }) => *s,
            _ => "d6",
        };
        
        let message = match fun::dice_faces(dice_type) {
            Some(faces) => {
                let result = fun::roll(faces);
                let embed = embeds::base()
                    .title("🎲 Se ha lanzado un dado")
                    .field("Dado", format!("d{faces}"), true)
                    .field("Resultado", result.to_string(), true);
                CreateInteractionResponseMessage::new().embed(embed)
            }
            None => CreateInteractionResponseMessage::new()
                .content("Dado no válido. Prueba con: d4, d6, d8, d10, d12, d20 o d100")
                .ephemeral(true),
        };

        cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await?;

        Ok(())
    }
}