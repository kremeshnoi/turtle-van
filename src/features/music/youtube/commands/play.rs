use serenity::prelude::TypeMapKey;
use songbird::input::{AuxMetadata, Input, YoutubeDl};

use crate::shared::{Context, Error, Errors};
use crate::HttpKey;
use crate::features::music::youtube::commands::shared::{
    get_user_voice_channel::get_user_voice_channel,
    join_voice_channel::join_voice_channel,
};

#[allow(dead_code)]
struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

#[poise::command(prefix_command, slash_command)]
pub async fn play(ctx: Context<'_>, #[rest] query: Option<String>) -> Result<(), Error> {
    ctx.say("Starting `play` command...").await?;

    let guild_id = ctx
        .guild_id()
        .ok_or(Errors::FAILED_TO_RETRIEVE_GUILD_ID)?;

    ctx.say("Guild ID retrieved.").await?;

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| Error::from(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT))?
        .clone();

    ctx.say("Voice client retrieved.").await?;

    let channel = match get_user_voice_channel(&ctx) {
        Some(c) => {
            ctx.say(format!("User is in voice channel: <#{c}>.")).await?;
            c
        }
        None => {
            ctx.say(Errors::FAILED_TO_JOIN_CHANNEL).await?;
            return Err(Error::from(Errors::FAILED_TO_JOIN_CHANNEL));
        }
    };

    ctx.say("Retrieving HTTP client...").await?;
    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .ok_or(Errors::FAILED_TO_RETRIEVE_HTTP)?
    };
    ctx.say("HTTP client retrieved.").await?;

    ctx.say(format!("Joining voice channel <#{channel}>...")).await?;
    let handler_lock = join_voice_channel(&voice_client, guild_id, channel).await?;
    ctx.say("Joined voice channel.").await?;

    let mut handler = handler_lock.lock().await;

    let query = match query {
        Some(q) => {
            ctx.say(format!("Searching and loading track for query: `{q}`")).await?;
            q
        }
        None => {
            ctx.say("Attempting to resume current track...").await?;
            return match handler.queue().current() {
                Some(track) => {
                    if let Err(e) = track.play() {
                        ctx.say("Failed to resume track.").await?;
                        Err(Error::from(e))
                    } else {
                        ctx.say("Track resumed.").await?;
                        Ok(())
                    }
                }
                None => {
                    ctx.say(Errors::FAILED_TO_PROVIDE_URL_TO_SONG_QUERY).await?;
                    Ok(())
                }
            };
        }
    };

    let src = YoutubeDl::new(http_client.clone(), query.clone());

    let mut input: Input = src.into();

    let _ = input.aux_metadata().await;

    ctx.say("Starting playback...").await?;
    let _track_handle = handler.play_input(input);
    ctx.say("Track has been added to the queue.").await?;

    Ok(())
}
