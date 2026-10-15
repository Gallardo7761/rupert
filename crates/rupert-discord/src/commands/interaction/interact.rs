use rupert_core::interaction::{self, GifClient};
use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateAutocompleteResponse, CreateCommand,
        CreateCommandOption, CreateEmbedFooter, CreateInteractionResponse, EditInteractionResponse,
        Mentionable,
    },
    async_trait,
};

use crate::{
    commands::{CommandResult, RCommand, options, reply},
    embeds,
};

/// Discord's limit for the name of an autocomplete choice.
const MAX_CHOICE_NAME: usize = 100;

pub struct Interact {
    gifs: GifClient,
}

impl Interact {
    pub fn new() -> Self {
        Self {
            gifs: GifClient::new(),
        }
    }
}

#[async_trait]
impl RCommand for Interact {
    fn name(&self) -> &str {
        "interact"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Interactúa con alguien, o expresa cómo te sientes")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "action", "Qué hacer")
                    .required(true)
                    .set_autocomplete(true),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "Con quién (algunas acciones no lo necesitan)",
            ))
    }

    async fn autocomplete(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let query = cmd.data.autocomplete().map_or("", |option| option.value);
        let response = interaction::suggest(query).into_iter().fold(
            CreateAutocompleteResponse::new(),
            |response, action| {
                let label = format!("{} — {}", action.name, action.description);
                response.add_string_choice(
                    label.chars().take(MAX_CHOICE_NAME).collect::<String>(),
                    action.name,
                )
            },
        );

        cmd.create_response(&ctx.http, CreateInteractionResponse::Autocomplete(response))
            .await?;
        Ok(())
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let (name, target) = {
            let options = cmd.data.options();
            (
                options::get_string(&options, "action"),
                options::get_user(&options, "user"),
            )
        };

        // The user can ignore the suggestions and type anything.
        let Some(action) = name.as_deref().and_then(interaction::find) else {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("No conozco esa acción: elige una de la lista"),
            )
            .await;
        };

        let author = cmd.user.mention().to_string();
        let target = target.map(|user| user.mention().to_string());
        let Some(sentence) = action.sentence(&author, target.as_deref()) else {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("Esa acción necesita a alguien: rellena `user`"),
            )
            .await;
        };

        // The GIF comes from an external API, which may take longer than 3 seconds.
        cmd.defer(&ctx.http).await?;

        let embed = embeds::base().description(sentence);
        match self.gifs.random(action.category).await {
            Ok(gif) => {
                let embed = embed.image(gif.url);
                let embed = match gif.anime {
                    Some(anime) => embed.footer(CreateEmbedFooter::new(format!("🎬 {anime}"))),
                    None => embed,
                };
                cmd.edit_response(&ctx.http, EditInteractionResponse::new().embed(embed))
                    .await?;
                Ok(())
            }
            // Without the GIF, the sentence still gets sent; the error is logged by Bot.
            Err(e) => {
                cmd.edit_response(&ctx.http, EditInteractionResponse::new().embed(embed))
                    .await?;
                Err(e.into())
            }
        }
    }
}
