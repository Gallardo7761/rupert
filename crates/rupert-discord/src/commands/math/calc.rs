use rupert_core::math::{MAX_EXPR_LEN, calc};
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

pub struct Calc;

#[async_trait]
impl RCommand for Calc {
    fn name(&self) -> &str {
        "calc"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Calcula el resultado de una expresión matemática")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "expression",
                    "Ej: 2+3*4 o (1+2)^3",
                )
                .required(true)
                .max_length(MAX_EXPR_LEN as u16),
            )
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let message = match options::get_string(&cmd.data.options(), "expression") {
            Some(expr) => match calc(&expr) {
                Ok(result) => {
                    let embed = embeds::base()
                        .title("🧮 Calculadora")
                        .field("Operación", expr, false)
                        .field("Resultado", result.to_string(), false);
                    CreateInteractionResponseMessage::new().embed(embed)
                }
                Err(e) => CreateInteractionResponseMessage::new()
                    .embed(embeds::base().description(format!("❌ {e}")))
                    .ephemeral(true),
            },
            None => CreateInteractionResponseMessage::new()
                .embed(embeds::base().description("❌ Necesito una expresión"))
                .ephemeral(true),
        };

        reply::send(ctx, cmd, message).await
    }
}
