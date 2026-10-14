use rupert_core::math::primality::{is_prime, next_prime};
use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
    },
    async_trait,
};

use crate::{
    commands::{CommandResult, RCommand, options, reply},
    embeds,
};

const MAX_NUMBER: u64 = 1_000_000_000_000;

pub struct Prime;

#[async_trait]
impl RCommand for Prime {
    fn name(&self) -> &str {
        "prime"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Comprobador y generador de primos")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "check",
                    "Comprueba si un número n es primo",
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::Integer, "n", "Número a comprobar")
                        .required(true)
                        .min_int_value(0)
                        .max_int_value(MAX_NUMBER),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "next",
                    "Devuelve el siguiente primo a n",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "n",
                        "Número a obtener su siguiente primo",
                    )
                    .required(true)
                    .min_int_value(0)
                    .max_int_value(MAX_NUMBER),
                ),
            )
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let options = cmd.data.options();

        let message = match options.first() {
            Some(ResolvedOption {
                name,
                value: ResolvedValue::SubCommand(inner),
                ..
            }) => {
                let number = options::get_integer(inner, "n").and_then(|n| u64::try_from(n).ok());

                match (*name, number) {
                    ("check", Some(n)) => check(n),
                    ("next", Some(n)) => next(n),
                    _ => error_message("Necesito un entero válido"),
                }
            }
            _ => error_message("Elige un subcomando: `check` o `next`"),
        };

        reply::send(ctx, cmd, message).await
    }
}

fn check(n: u64) -> CreateInteractionResponseMessage {
    let verdict = if is_prime(n) {
        "✅ Es primo"
    } else {
        "❌ No es primo"
    };
    let embed = embeds::base()
        .title("🔢 Comprobador de primos")
        .field("Número", n.to_string(), true)
        .field("Resultado", verdict, true);
    CreateInteractionResponseMessage::new().embed(embed)
}

fn next(n: u64) -> CreateInteractionResponseMessage {
    match next_prime(n) {
        Some(p) => {
            let embed = embeds::base()
                .title("🔢 Siguiente primo")
                .field("Desde", n.to_string(), true)
                .field("Siguiente primo", p.to_string(), true);
            CreateInteractionResponseMessage::new().embed(embed)
        }
        None => error_message("No existe un siguiente primo que quepa en 64 bits"),
    }
}

fn error_message(text: &str) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .content(text)
        .ephemeral(true)
}
