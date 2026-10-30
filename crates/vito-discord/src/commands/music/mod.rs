pub mod leave;
pub mod pause;
pub mod play;
pub mod queue;
pub mod resume;
pub mod skip;
pub mod stop;

use std::collections::HashMap;
use std::sync::Arc;

use vito_core::logging::{VLogger, VLoggerContext};
use vito_core::music::queue::Queue;
use vito_core::music::track::{Track, format_duration};
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateEmbed, CreateInteractionResponseMessage, GuildId,
    UserId,
};
use serenity::async_trait;
use serenity::prelude::TypeMapKey;
use songbird::input::YoutubeDl;
use songbird::tracks::{PlayMode, TrackHandle, TrackResult};
use songbird::{Event, EventContext, Songbird, TrackEvent};
use tokio::sync::Mutex;

use crate::commands::{CommandResult, reply};
use crate::embeds;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = reqwest::Client;
}

#[derive(Default)]
pub struct Player {
    pub queue: Queue,
    pub handle: Option<TrackHandle>,
}

pub type Players = Arc<Mutex<HashMap<GuildId, Player>>>;

pub struct PlayersKey;

impl TypeMapKey for PlayersKey {
    type Value = Players;
}

pub fn guild_id(cmd: &CommandInteraction) -> CommandResult<GuildId> {
    cmd.guild_id
        .ok_or_else(|| "music command used outside a server".into())
}

pub async fn http(ctx: &Context) -> CommandResult<reqwest::Client> {
    ctx.data
        .read()
        .await
        .get::<HttpKey>()
        .cloned()
        .ok_or_else(|| "HTTP client not in the TypeMap".into())
}

pub async fn players(ctx: &Context) -> CommandResult<Players> {
    ctx.data
        .read()
        .await
        .get::<PlayersKey>()
        .cloned()
        .ok_or_else(|| "players not in the TypeMap".into())
}

pub async fn manager(ctx: &Context) -> CommandResult<Arc<Songbird>> {
    songbird::get(ctx)
        .await
        .ok_or_else(|| "songbird not registered".into())
}

pub fn user_channel(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Option<ChannelId> {
    let guild = ctx.cache.guild(guild_id)?;
    guild.voice_states.get(&user_id)?.channel_id
}

pub async fn bot_channel(manager: &Songbird, guild_id: GuildId) -> Option<songbird::id::ChannelId> {
    let call = manager.get(guild_id)?;
    call.lock().await.current_channel()
}

pub async fn listener_guild(
    ctx: &Context,
    cmd: &CommandInteraction,
) -> CommandResult<Option<GuildId>> {
    let guild_id = guild_id(cmd)?;
    let manager = manager(ctx).await?;

    let Some(bot) = bot_channel(&manager, guild_id).await else {
        reply::send(
            ctx,
            cmd,
            reply::error_message("No estoy en ningún canal de voz"),
        )
        .await?;
        return Ok(None);
    };

    let user = user_channel(ctx, guild_id, cmd.user.id).map(songbird::id::ChannelId::from);
    if user != Some(bot) {
        reply::send(
            ctx,
            cmd,
            reply::error_message("Tienes que estar en mi canal de voz"),
        )
        .await?;
        return Ok(None);
    }

    Ok(Some(guild_id))
}

pub async fn control(
    ctx: &Context,
    cmd: &CommandInteraction,
    action: fn(&TrackHandle) -> TrackResult<()>,
    done: &str,
) -> CommandResult {
    let Some(guild_id) = listener_guild(ctx, cmd).await? else {
        return Ok(());
    };

    let playing = {
        let players = players(ctx).await?;
        let players = players.lock().await;
        players.get(&guild_id).and_then(|player| {
            let title = player.queue.current()?.title().to_string();
            Some((player.handle.clone()?, title))
        })
    };
    let Some((handle, title)) = playing else {
        return reply::send(ctx, cmd, reply::error_message("No está sonando nada")).await;
    };

    action(&handle)?;
    let embed = embeds::base().description(format!("{done}: **{title}**"));
    reply::send(
        ctx,
        cmd,
        CreateInteractionResponseMessage::new().embed(embed),
    )
    .await
}

#[derive(Clone)]
pub struct Playback {
    pub manager: Arc<Songbird>,
    pub http: reqwest::Client,
    pub players: Players,
    pub guild_id: GuildId,
}

impl Playback {
    pub async fn new(ctx: &Context, guild_id: GuildId) -> CommandResult<Self> {
        Ok(Self {
            manager: manager(ctx).await?,
            http: http(ctx).await?,
            players: players(ctx).await?,
            guild_id,
        })
    }

    pub async fn play(&self, track: &Track) -> CommandResult {
        let call = self
            .manager
            .get(self.guild_id)
            .ok_or("not in a voice channel")?;

        VLogger::new(VLoggerContext::Discord).info(&format!(
            "Playing \"{}\" <{}>",
            track.title(),
            track.url()
        ));
        let source = YoutubeDl::new(self.http.clone(), track.url().to_string());
        let handle = call.lock().await.play_input(source.into());
        handle.add_event(Event::Track(TrackEvent::End), TrackEnd(self.clone()))?;
        handle.add_event(
            Event::Track(TrackEvent::Error),
            TrackError(track.title().to_string()),
        )?;

        self.players
            .lock()
            .await
            .entry(self.guild_id)
            .or_default()
            .handle = Some(handle);
        Ok(())
    }
}

struct TrackEnd(Playback);

#[async_trait]
impl songbird::EventHandler for TrackEnd {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let next = {
            let mut players = self.0.players.lock().await;
            let player = players.get_mut(&self.0.guild_id)?;
            player.handle = None;
            player.queue.advance().cloned()
        };

        if let Some(track) = next
            && let Err(e) = self.0.play(&track).await
        {
            VLogger::new(VLoggerContext::Discord).error("Error playing next track", e.as_ref());
        }
        None
    }
}

struct TrackError(String);

#[async_trait]
impl songbird::EventHandler for TrackError {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(tracks) = ctx {
            for (state, _) in *tracks {
                if let PlayMode::Errored(e) = &state.playing {
                    VLogger::new(VLoggerContext::Discord)
                        .error(&format!("Error playing \"{}\"", self.0), e);
                }
            }
        }
        None
    }
}

pub fn track_embed(track: &Track) -> CreateEmbed {
    let duration = track
        .duration()
        .map_or("🔴 En directo".to_string(), format_duration);

    let embed = embeds::base()
        .description(format!("**[{}]({})**", track.title(), track.url()))
        .field("Autor", track.author(), true)
        .field("Duración", duration, true)
        .field("Pedida por", track.requested_by(), true);

    match track.thumbnail() {
        Some(thumbnail) => embed.thumbnail(thumbnail),
        None => embed,
    }
}
