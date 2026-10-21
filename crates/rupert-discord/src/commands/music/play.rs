use rupert_core::music::query::{self, Query};
use rupert_core::music::track::Track;
use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        EditInteractionResponse,
    },
    async_trait,
};
use songbird::input::{Compose, YoutubeDl};

use super::{Playback, bot_channel, guild_id, track_embed, user_channel};
use crate::{
    commands::{CommandResult, RCommand, options, reply},
    embeds,
};

pub struct Play;

#[async_trait]
impl RCommand for Play {
    fn name(&self) -> &str {
        "play"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Pone una canción de YouTube o la añade a la cola")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "query",
                    "Enlace de YouTube o lo que quieras buscar",
                )
                .required(true),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "next",
                "Ponerla la siguiente en vez de al final de la cola",
            ))
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let (input, next) = {
            let options = cmd.data.options();
            (
                options::get_string(&options, "query").unwrap_or_default(),
                options::get_bool(&options, "next").unwrap_or(false),
            )
        };

        let guild_id = guild_id(cmd)?;
        let Some(channel_id) = user_channel(ctx, guild_id, cmd.user.id) else {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("Métete en un canal de voz primero"),
            )
            .await;
        };

        let playback = Playback::new(ctx, guild_id).await?;
        let bot = bot_channel(&playback.manager, guild_id).await;
        let busy = {
            let mut players = playback.players.lock().await;
            if bot.is_none() {
                players.remove(&guild_id);
            }
            players
                .get(&guild_id)
                .is_some_and(|player| !player.queue.is_empty())
        };
        if busy && bot.is_some_and(|bot| bot != channel_id.into()) {
            return reply::send(
                ctx,
                cmd,
                reply::error_message("Estoy poniendo música en otro canal"),
            )
            .await;
        }

        cmd.defer(&ctx.http).await?;

        if bot != Some(channel_id.into())
            && let Err(e) = playback.manager.join(guild_id, channel_id).await
        {
            edit_error(ctx, cmd, "No he podido entrar en tu canal de voz").await?;
            return Err(e.into());
        }

        let track = match fetch(&playback, &input, cmd.user.display_name()).await {
            Ok(track) => track,
            Err(e) => {
                edit_error(ctx, cmd, "No he podido cargar esa canción").await?;
                return Err(e);
            }
        };

        let position = {
            let mut players = playback.players.lock().await;
            let queue = &mut players.entry(guild_id).or_default().queue;
            if next {
                queue.insert(track.clone())
            } else {
                queue.push(track.clone())
            }
        };

        let title = if position == 0 {
            playback.play(&track).await?;
            "▶️ Sonando".to_string()
        } else {
            format!("➕ En la cola (posición {position})")
        };

        let embed = track_embed(&track).title(title);
        cmd.edit_response(&ctx.http, EditInteractionResponse::new().embed(embed))
            .await?;
        Ok(())
    }
}

async fn edit_error(ctx: &Context, cmd: &CommandInteraction, text: &str) -> CommandResult {
    let embed = embeds::base().description(format!("❌ {text}"));
    cmd.edit_response(&ctx.http, EditInteractionResponse::new().embed(embed))
        .await?;
    Ok(())
}

async fn fetch(playback: &Playback, input: &str, requested_by: &str) -> CommandResult<Track> {
    let mut source = match query::parse(input) {
        Query::Url(url) => YoutubeDl::new(playback.http.clone(), url),
        Query::Search(search) => YoutubeDl::new_search(playback.http.clone(), search),
    };
    let metadata = source.aux_metadata().await?;

    let url = metadata.source_url.ok_or("yt-dlp returned no URL")?;
    let author = metadata
        .artist
        .or(metadata.channel)
        .unwrap_or_else(|| "Desconocido".to_string());

    Ok(Track::new(
        metadata.title.unwrap_or_else(|| "Sin título".to_string()),
        url,
        metadata.thumbnail,
        author,
        metadata.duration,
        requested_by.to_string(),
    ))
}
