use tracing::{error, info};

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let vc = VoiceContext::from_ctx(&ctx).await?;
    vc.require_call()?;

    if let Some(token) = ctx
        .data()
        .music_youtube
        .playlist_cancel_tokens
        .read()
        .await
        .get(&vc.guild_id)
    {
        token.cancel();
        info!(guild_id = ?vc.guild_id, "Cancelled playlist loading");
    }

    info!(guild_id = ?vc.guild_id, "Leaving voice channel");
    vc.voice_client
        .remove(vc.guild_id)
        .await
        .inspect_err(
            |e| error!(guild_id = ?vc.guild_id, error = %e, "Failed to leave voice channel"),
        )
        .map_err(|_| MusicYoutubeError::LeaveChannelFailed)?;
    info!("Left voice channel");
    ctx.say(MusicYoutubeMessage::LEFT_CHANNEL).await?;

    Ok(())
}
