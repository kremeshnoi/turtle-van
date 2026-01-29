use tracing::info;

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext, join_voice_channel};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let vc = VoiceContext::from_ctx(&ctx).await?;
    let user_channel = VoiceContext::require_user_channel(&ctx)
        .map_err(|_| MusicYoutubeError::JoinChannelFailed)?;

    info!(guild_id = ?vc.guild_id, channel_id = ?user_channel, "Joining voice channel");
    join_voice_channel(&vc.voice_client, vc.guild_id, user_channel).await?;
    info!("Joined voice channel");
    ctx.say(MusicYoutubeMessage::JOINED_CHANNEL).await?;

    Ok(())
}
