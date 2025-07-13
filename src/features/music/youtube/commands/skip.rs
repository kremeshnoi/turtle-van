
use crate::shared::{Context, Error, Errors};
use crate::features::music::youtube::commands::shared::{
    get_user_voice_channel::get_user_voice_channel,
};

#[poise::command(prefix_command, slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("⏭️ Executing `skip` command...").await?;

    let guild_id = ctx
        .guild_id()
        .ok_or(Errors::FAILED_TO_RETRIEVE_GUILD_ID)?;

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| Error::from(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT))?
        .clone();

    match get_user_voice_channel(&ctx) {
        Some(c) => c,
        None => {
            ctx.say("❌ You must be in a voice channel to use this command.").await?;
            return Ok(());
        }
    };

    if let Some(call) = voice_client.get(guild_id) {
        let handler = call.lock().await;

        if let Some(track) = handler.queue().current() {
            track.stop()?;
            ctx.say("⏭️ Skipped current track.").await?;
        } else {
            ctx.say("ℹ️ No track is currently playing.").await?;
        }
    } else {
        ctx.say("❌ Not currently in a voice channel.").await?;
    }

    Ok(())
}