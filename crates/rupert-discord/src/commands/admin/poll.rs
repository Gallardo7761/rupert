use std::time::Duration;

use serenity::{
    all::{
        ChannelType, CommandInteraction, CommandOptionType, Context, CreateCommand,
        CreateCommandOption, CreateInputText, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage, CreatePoll, CreatePollAnswer,
        CreateQuickModal, InputTextStyle, Permissions,
    },
    async_trait,
};

use crate::commands::{CommandResult, RCommand, options, reply};

const MIN_ANSWERS: usize = 2;
const MAX_ANSWERS: usize = 10;
const MAX_QUESTION: u16 = 300;
const MAX_ANSWER: usize = 55;

fn parse_answers(raw: &str) -> Result<Vec<String>, String> {
    let answers: Vec<String> = raw
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();

    if !(MIN_ANSWERS..=MAX_ANSWERS).contains(&answers.len()) {
        return Err(format!(
            "Necesito entre {MIN_ANSWERS} y {MAX_ANSWERS} respuestas, una por línea"
        ));
    }

    if answers.iter().any(|a| a.chars().count() > MAX_ANSWER) {
        return Err(format!(
            "Cada respuesta puede tener como mucho {MAX_ANSWER} caracteres"
        ));
    }

    Ok(answers)
}

pub struct Poll;

#[async_trait]
impl RCommand for Poll {
    fn name(&self) -> &str {
        "poll"
    }

    fn permission(&self) -> Option<Permissions> {
        Some(Permissions::MANAGE_GUILD)
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Crea una votación")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "duration",
                    "Cuánto dura (por defecto, 1 día)",
                )
                .add_int_choice("1 hora", 1)
                .add_int_choice("6 horas", 6)
                .add_int_choice("1 día", 24)
                .add_int_choice("3 días", 72)
                .add_int_choice("1 semana", 168),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "multiple",
                "Permitir elegir varias respuestas",
            ))
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "Dónde publicarla (por defecto, este canal)",
                )
                .channel_types(vec![ChannelType::Text, ChannelType::News]),
            )
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let (duration, multiple, channel) = {
            let options = cmd.data.options();
            (
                options::get_integer(&options, "duration"),
                options::get_bool(&options, "multiple"),
                options::get_channel(&options, "channel"),
            )
        };

        let hours = duration.unwrap_or(24).clamp(1, 168) as u64;
        let multiple = multiple.unwrap_or(false);
        let channel = channel.unwrap_or(cmd.channel_id);

        let modal = CreateQuickModal::new("Nueva encuesta")
            .timeout(Duration::from_secs(600))
            .field(
                CreateInputText::new(InputTextStyle::Short, "Pregunta", "")
                    .max_length(MAX_QUESTION),
            )
            .field(
                CreateInputText::new(InputTextStyle::Paragraph, "Respuestas (una por línea)", "")
                    .max_length(600),
            );

        let Some(response) = cmd.quick_modal(ctx, modal).await? else {
            return Ok(());
        };

        let confirmation = match response.inputs.as_slice() {
            [question, raw] => match parse_answers(raw) {
                Ok(answers) => {
                    let poll = CreatePoll::new()
                        .question(question)
                        .answers(
                            answers
                                .into_iter()
                                .map(|a| CreatePollAnswer::new().text(a))
                                .collect(),
                        )
                        .duration(Duration::from_secs(hours * 3600));

                    let poll = if multiple {
                        poll.allow_multiselect()
                    } else {
                        poll
                    };

                    match channel
                        .send_message(&ctx.http, CreateMessage::new().poll(poll))
                        .await
                    {
                        Ok(_) => CreateInteractionResponseMessage::new()
                            .content(format!("✅ Votación publicada en <#{channel}>"))
                            .ephemeral(true),
                        Err(_) => reply::error_message(&format!(
                            "No he podido escribir en <#{channel}>. Revisa mis permisos: \
                            Ver canal, Enviar mensajes e Insertar enlaces"
                        )),
                    }
                }
                Err(msg) => reply::error_message(&msg),
            },
            _ => reply::error_message("El formulario llegó incompleto"),
        };

        response
            .interaction
            .create_response(&ctx.http, CreateInteractionResponse::Message(confirmation))
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_answer_per_line() {
        assert_eq!(
            parse_answers("Sí\nNo"),
            Ok(vec!["Sí".to_string(), "No".to_string()])
        );
    }

    #[test]
    fn trims_and_skips_blank_lines() {
        assert_eq!(
            parse_answers("  Sí \n\n  No  \n"),
            Ok(vec!["Sí".to_string(), "No".to_string()])
        );
    }

    #[test]
    fn needs_at_least_two_answers() {
        assert!(parse_answers("").is_err());
        assert!(parse_answers("Solo una").is_err());
    }

    #[test]
    fn rejects_more_than_the_maximum() {
        let raw = (1..=MAX_ANSWERS + 1)
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(parse_answers(&raw).is_err());
    }

    #[test]
    fn rejects_long_answers_but_accepts_the_limit() {
        let ok = "a".repeat(MAX_ANSWER);
        let long = "a".repeat(MAX_ANSWER + 1);
        assert!(parse_answers(&format!("{ok}\nb")).is_ok());
        assert!(parse_answers(&format!("{long}\nb")).is_err());
    }
}
