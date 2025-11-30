use serenity::prelude::TypeMapKey;
use songbird::input::{AuxMetadata, Input, YoutubeDl};
use std::process::Command;
use tokio::task;

use super::shared::Errors;
use crate::HttpKey;
use crate::features::music::youtube::commands::shared::{
    get_user_voice_channel_id::get_user_voice_channel_id, join_voice_channel::join_voice_channel,
};
use crate::shared::{Context, Error};

#[allow(dead_code)]
struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

#[poise::command(prefix_command, slash_command)]
pub async fn play(ctx: Context<'_>, #[rest] query: Option<String>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Errors::FAILED_TO_RETRIEVE_GUILD_ID)?;

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| Error::from(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT))?
        .clone();

    let channel = match get_user_voice_channel_id(&ctx) {
        Ok(c) => c,
        Err(_) => {
            return Err(Error::from(Errors::FAILED_TO_JOIN_CHANNEL));
        }
    };

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .ok_or(Errors::FAILED_TO_RETRIEVE_HTTP)?
    };

    let handler_lock = join_voice_channel(&voice_client, guild_id, channel).await?;

    let mut handler = handler_lock.lock().await;

    let query = match query {
        Some(q) => {
            ctx.say(format!("Searching and loading track for query: `{q}`"))
                .await?;
            q
        }
        None => {
            return match handler.queue().current() {
                Some(track) => {
                    if let Err(e) = track.play() {
                        Err(Error::from(e))
                    } else {
                        Ok(())
                    }
                }
                None => Ok(()),
            };
        }
    };

    if query.starts_with("http") && (query.contains("playlist") || query.contains("list=")) {
        let playlist_urls = extract_playlist_urls(&query).await?;

        if playlist_urls.is_empty() {
            ctx.say("No videos found in playlist").await?;
            return Ok(());
        }

        ctx.say(format!(
            "Loading {} tracks from playlist in background...",
            playlist_urls.len()
        ))
        .await?;

        let handler_clone = handler_lock.clone();
        let http_client_clone = http_client.clone();
        let ctx_id = ctx.channel_id();
        let ctx_http = ctx.serenity_context().http.clone();

        drop(handler);

        task::spawn(async move {
            let mut handler = handler_clone.lock().await;

            for url in playlist_urls {
                let src = YoutubeDl::new(http_client_clone.clone(), url);
                let mut input: Input = src.into();

                if let Ok(metadata) = input.aux_metadata().await {
                    let track_handle = handler.enqueue_input(input).await;

                    track_handle
                        .typemap()
                        .write()
                        .await
                        .insert::<TrackMetaKey>(metadata.clone());
                }
            }
        });
    } else {
        let src = if query.starts_with("http") {
            YoutubeDl::new(http_client.clone(), query.clone())
        } else {
            YoutubeDl::new_search(http_client.clone(), query.clone())
        };

        let mut input: Input = src.into();

        let metadata = input.aux_metadata().await?.clone();

        let track_handle = handler.enqueue_input(input).await;

        let track_info = format!(
            "{} - {} ({})",
            metadata
                .clone()
                .title
                .unwrap_or_else(|| "Unknown Title".to_string()),
            metadata
                .clone()
                .artist
                .unwrap_or_else(|| "Unknown Artist".to_string()),
            metadata
                .clone()
                .duration
                .map_or("Unknown Duration".to_string(), |d| {
                    format!("{}:{:02}", d.as_secs() / 60, d.as_secs() % 60)
                })
        );

        track_handle
            .typemap()
            .write()
            .await
            .insert::<TrackMetaKey>(metadata);

        ctx.say(format!("`{track_info}` has been added to the queue"))
            .await?;
    }

    Ok(())
}

async fn extract_playlist_urls(playlist_url: &str) -> Result<Vec<String>, Error> {
    let output = Command::new("yt-dlp")
        .args(&["--flat-playlist", "--print", "url", playlist_url])
        .output()
        .map_err(|e| Error::from(format!("Failed to execute yt-dlp: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::from(format!("yt-dlp failed: {}", stderr)));
    }

    let urls = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(urls)
}
