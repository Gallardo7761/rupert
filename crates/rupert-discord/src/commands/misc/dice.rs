use rupert_core::fun;
use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateInteractionResponseMessage,
    },
    async_trait,
};

use crate::{
    commands::{CommandResult, RCommand, options, reply},
    embeds,
};

pub struct Dice;

#[async_trait]
impl RCommand for Dice {
    fn name(&self) -> &str {
        "dice"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Tira un dado")
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "type",
                "Tipo de dado. Ej: d6, d4, d10",
            ))
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let dice_type =
            options::get_string(&cmd.data.options(), "type").unwrap_or_else(|| "d6".to_string());

        let message = match fun::dice_faces(&dice_type) {
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

        reply::send(ctx, cmd, message).await
    }
}
