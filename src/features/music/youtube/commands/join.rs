use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext, join_voice_channel};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let vc = VoiceContext::from_ctx(&ctx).await?;
    let user_channel = VoiceContext::require_user_channel(&ctx)
        .map_err(|_| MusicYoutubeError::JoinChannelFailed)?;

    join_voice_channel(&vc.voice_client, vc.guild_id, user_channel).await?;
    ctx.say(MusicYoutubeMessage::JOINED_CHANNEL).await?;

    Ok(())
}
