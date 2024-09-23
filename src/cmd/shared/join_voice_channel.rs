use std::sync::Arc;
use songbird::Call;
use tokio::sync::Mutex;
use crate::types::Error;

pub async fn join_voice_channel(
    voice_client: &Arc<songbird::Songbird>,
    guild_id: poise::serenity_prelude::GuildId,
    channel_id: poise::serenity_prelude::ChannelId,
) -> Result<Arc<Mutex<Call>>, Error> {
    voice_client.join(guild_id, channel_id).await.map_err(|error| Box::new(error) as Error)
}