use songbird::events::{Event, TrackEvent};
use songbird::input::{Input, YoutubeDl};
use songbird::tracks::Track;
use std::process::Command;
use std::sync::Arc;
use tokio::task;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use super::shared::{
    MusicYoutubeError, MusicYoutubeMessage, TrackDisplayInfo, TrackPlayNotifier, VoiceContext,
    join_voice_channel,
};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn play(ctx: Context<'_>, #[rest] query: Option<String>) -> Result<(), Error> {
    ctx.defer().await?;

    let vc = VoiceContext::from_ctx(&ctx).await?;
    let user_channel = VoiceContext::require_user_channel(&ctx)
        .map_err(|_| MusicYoutubeError::JoinChannelFailed)?;

    let http_client = ctx.data().http_client.clone();

    let handler_lock = join_voice_channel(&vc.voice_client, vc.guild_id, user_channel).await?;
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
                    track.play()?;
                    ctx.say(MusicYoutubeMessage::RESUMED_TRACK).await?;
                    Ok(())
                }
                None => {
                    ctx.say(MusicYoutubeError::NoTrackPlaying.to_string())
                        .await?;
                    Ok(())
                }
            };
        }
    };

    if query.starts_with("http") && (query.contains("playlist") || query.contains("list=")) {
        info!(url = %query, "Extracting playlist URLs");
        let playlist_urls = extract_playlist_urls(&query).await?;
        info!(count = playlist_urls.len(), "Playlist URLs extracted");

        if playlist_urls.is_empty() {
            ctx.say(MusicYoutubeError::NoVideosInPlaylist.to_string())
                .await?;
            return Ok(());
        }

        let capped = playlist_urls.len() >= MAX_PLAYLIST_TRACKS;
        ctx.say(if capped {
            format!("Loading first {MAX_PLAYLIST_TRACKS} tracks from playlist (limit reached)...")
        } else {
            format!(
                "Loading {} tracks from playlist in background...",
                playlist_urls.len()
            )
        })
        .await?;

        let handler_clone = handler_lock.clone();
        let http_client_clone = http_client.clone();
        let channel_id = ctx.channel_id();
        let serenity_http = Arc::clone(&ctx.serenity_context().http);

        let cancel_tokens = ctx.data().music_youtube.playlist_cancel_tokens.clone();
        {
            let tokens = cancel_tokens.read().await;
            if let Some(existing_token) = tokens.get(&vc.guild_id) {
                existing_token.cancel();
            }
        }

        let cancel_token = CancellationToken::new();
        {
            let mut tokens = cancel_tokens.write().await;
            tokens.insert(vc.guild_id, cancel_token.clone());
        }

        drop(handler);

        task::spawn(async move {
            for url in playlist_urls {
                if cancel_token.is_cancelled() {
                    break;
                }

                let src =
                    YoutubeDl::new(http_client_clone.clone(), url).user_args(prefer_https_args());
                let mut input: Input = src.into();

                match input.aux_metadata().await {
                    Err(e) => {
                        warn!(error = ?e, "Failed to fetch metadata for playlist track");
                        continue;
                    }
                    Ok(metadata) => {
                        if cancel_token.is_cancelled() {
                            break;
                        }

                        let track = Track::new_with_data(input, Arc::new(metadata));
                        let mut handler = handler_clone.lock().await;
                        let track_handle = handler.enqueue(track).await;

                        let _ = track_handle.add_event(
                            Event::Track(TrackEvent::Play),
                            TrackPlayNotifier {
                                channel_id,
                                http: Arc::clone(&serenity_http),
                            },
                        );
                    }
                }
            }

            let mut tokens = cancel_tokens.write().await;
            tokens.remove(&vc.guild_id);
        });
    } else {
        let src = if query.starts_with("http") {
            YoutubeDl::new(http_client.clone(), query.clone()).user_args(prefer_https_args())
        } else {
            YoutubeDl::new_search(http_client.clone(), query.clone()).user_args(prefer_https_args())
        };

        let mut input: Input = src.into();

        let metadata = match input.aux_metadata().await {
            Ok(m) => m.clone(),
            Err(e) => {
                error!(error = ?e, "Failed to fetch metadata");
                ctx.say(MusicYoutubeError::NoResultsFound.to_string())
                    .await?;
                return Ok(());
            }
        };

        let queued = !handler.queue().is_empty();

        let track = Track::new_with_data(input, Arc::new(metadata.clone()));
        let track_handle = handler.enqueue(track).await;
        info!(queue_length = handler.queue().len(), "Track enqueued");

        track_handle.add_event(
            Event::Track(TrackEvent::Play),
            TrackPlayNotifier {
                channel_id: ctx.channel_id(),
                http: Arc::clone(&ctx.serenity_context().http),
            },
        )?;

        if queued {
            ctx.say(format!(
                "{} has been added to the queue",
                TrackDisplayInfo::from_metadata(&metadata).message()
            ))
            .await?;
        }
    }

    Ok(())
}

const MAX_PLAYLIST_TRACKS: usize = 300;

fn prefer_https_args() -> Vec<String> {
    vec!["-S".to_string(), "proto:https".to_string()]
}

async fn extract_playlist_urls(playlist_url: &str) -> Result<Vec<String>, Error> {
    let output = Command::new("yt-dlp")
        .args([
            "--flat-playlist",
            "--print",
            "webpage_url",
            "--playlist-end",
            &MAX_PLAYLIST_TRACKS.to_string(),
            playlist_url,
        ])
        .output()
        .map_err(|e| MusicYoutubeError::YtDlpExecutionFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(MusicYoutubeError::YtDlpFailed(stderr.to_string()).into());
    }

    let urls = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(urls)
}
