use serenity::prelude::TypeMapKey;
use songbird::input::{AuxMetadata, Input, YoutubeDl};
use std::sync::Arc;
use tokio::sync::RwLock;
use typemap_rev::TypeMap;

use crate::consts::{
    FAILED, FAILED_TO_JOIN_CHANNEL, FAILED_TO_PROVIDE_URL_TO_SONG_QUERY,
    FAILED_TO_RETRIEVE_GUILD_ID, FAILED_TO_RETRIEVE_HTTP, FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT,
};
use crate::types::{Context, Error};

use crate::HttpKey;
use crate::cmd::shared::get_user_voice_channel::get_user_voice_channel;
use crate::cmd::shared::join_voice_channel::join_voice_channel;

struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

#[derive(Default)]
pub struct MyTrackData(pub RwLock<TypeMap>);

#[poise::command(prefix_command, slash_command)]
pub async fn play(ctx: Context<'_>, #[rest] query: Option<String>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect(FAILED_TO_RETRIEVE_GUILD_ID);
    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| Error::from(FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT))?
        .clone();

    match get_user_voice_channel(&ctx) {
        Some(channel) => {
            let http_client = {
                let data = ctx.serenity_context().data.read().await;
                data.get::<HttpKey>()
                    .cloned()
                    .expect(FAILED_TO_RETRIEVE_HTTP)
            };

            let handler_lock = join_voice_channel(&voice_client, guild_id, channel).await?;
            let mut handler = handler_lock.lock().await;

            let query = match query {
                Some(q) => q,
                None => {
                    return match handler.queue().current() {
                        Some(track) => {
                            track.play().expect(FAILED);
                            Ok(())
                        }
                        None => {
                            ctx.say(FAILED_TO_PROVIDE_URL_TO_SONG_QUERY).await?;
                            Ok(())
                        }
                    };
                }
            };

            let src = if query.starts_with("http") {
                YoutubeDl::new_ytdl_like("./yt-dlp", http_client, query)
            } else {
                YoutubeDl::new_search_ytdl_like("./yt-dlp", http_client, query)
            };

            let mut input: Input = src.into();
            let metadata = input.aux_metadata().await?.clone();

            let track_handle = handler.enqueue_input(input).await;

            {
                let my_track_data: Arc<MyTrackData> = track_handle.data::<MyTrackData>();
                let mut data_lock = my_track_data.0.write().await;
                data_lock.insert::<TrackMetaKey>(metadata);
            }

            Ok(())
        }
        None => {
            ctx.say(FAILED_TO_JOIN_CHANNEL).await?;
            Err(Error::from(FAILED_TO_JOIN_CHANNEL))
        }
    }
}
