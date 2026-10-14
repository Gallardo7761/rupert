use std::time::Duration;

use serenity::{
    all::{
        ChannelType, CommandInteraction, CommandOptionType, Context, CreateCommand,
        CreateCommandOption, CreateEmbedAuthor, CreateEmbedFooter, CreateInputText,
        CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, InputTextStyle,
        Permissions, Timestamp,
    },
    async_trait,
    utils::CreateQuickModal,
};

use crate::{
    commands::{CommandResult, RCommand, options, reply},
    embeds,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Level {
    Info,
    News,
    Warning,
    Urgent,
}

impl Level {
    const ALL: [Level; 4] = [Level::Info, Level::News, Level::Warning, Level::Urgent];

    fn value(self) -> &'static str {
        match self {
            Level::Info => "info",
            Level::News => "novedad",
            Level::Warning => "aviso",
            Level::Urgent => "urgente",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Level::Info => "Información",
            Level::News => "Novedad",
            Level::Warning => "Aviso",
            Level::Urgent => "Urgente",
        }
    }

    fn emoji(self) -> &'static str {
        match self {
            Level::Info => "ℹ️",
            Level::News => "✨",
            Level::Warning => "⚠️",
            Level::Urgent => "🚨",
        }
    }

    fn color(self) -> u32 {
        match self {
            Level::Info => 0x3498DB,
            Level::News => 0xF74C00,
            Level::Warning => 0xF1C40F,
            Level::Urgent => 0xE74C3C,
        }
    }

    fn from_value(value: &str) -> Option<Level> {
        Level::ALL.into_iter().find(|l| l.value() == value)
    }
}

pub struct Say;

#[async_trait]
impl RCommand for Say {
    fn name(&self) -> &str {
        "say"
    }

    fn permission(&self) -> Option<Permissions> {
        Some(Permissions::MANAGE_GUILD)
    }

    fn register(&self) -> CreateCommand {
        let level_option = Level::ALL.into_iter().fold(
            CreateCommandOption::new(
                CommandOptionType::String,
                "level",
                "Importancia del anuncio",
            )
            .required(true),
            |opt, level| {
                opt.add_string_choice(
                    format!("{} {}", level.emoji(), level.label()),
                    level.value(),
                )
            },
        );

        CreateCommand::new(self.name())
            .description("Publica un anuncio")
            .add_option(level_option)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "Dónde publicarlo (por defecto, este canal)",
                )
                .channel_types(vec![ChannelType::Text, ChannelType::News]),
            )
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let (level, channel) = {
            let options = cmd.data.options();
            (
                options::get_string(&options, "level").and_then(|v| Level::from_value(&v)),
                options::get_channel(&options, "channel"),
            )
        };

        let Some(level) = level else {
            return reply::send(ctx, cmd, reply::error_message("Nivel de anuncio no válido")).await;
        };
        let channel = channel.unwrap_or(cmd.channel_id);

        let modal = CreateQuickModal::new("Nuevo anuncio")
            .timeout(Duration::from_secs(600))
            .field(CreateInputText::new(InputTextStyle::Short, "Título", "").max_length(100))
            .field(CreateInputText::new(InputTextStyle::Paragraph, "Mensaje", "").max_length(4000));

        let Some(response) = cmd.quick_modal(ctx, modal).await? else {
            return Ok(());
        };

        let confirmation = match response.inputs.as_slice() {
            [title, body] => {
                let embed = embeds::base()
                    .color(level.color())
                    .author(CreateEmbedAuthor::new(format!(
                        "{} {}",
                        level.emoji(),
                        level.label().to_uppercase()
                    )))
                    .title(title)
                    .description(body)
                    .footer(CreateEmbedFooter::new(format!(
                        "Enviado por {}",
                        cmd.user.name
                    )))
                    .timestamp(Timestamp::now());

                match channel
                    .send_message(&ctx.http, CreateMessage::new().embed(embed))
                    .await
                {
                    Ok(_) => CreateInteractionResponseMessage::new()
                        .content(format!("✅ Anuncio publicado en <#{channel}>"))
                        .ephemeral(true),
                    Err(_) => reply::error_message(&format!(
                        "No he podido escribir en <#{channel}>. Revisa mis permisos: \
                         Ver canal, Enviar mensajes e Insertar enlaces"
                    )),
                }
            }
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
    fn every_level_roundtrips() {
        for level in Level::ALL {
            assert_eq!(Level::from_value(level.value()), Some(level));
        }
    }

    #[test]
    fn level_values_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for level in Level::ALL {
            assert!(seen.insert(level.value()), "{} repetido", level.value());
        }
    }

    #[test]
    fn unknown_levels_are_rejected() {
        assert_eq!(Level::from_value(""), None);
        assert_eq!(Level::from_value("nope"), None);
    }
}
