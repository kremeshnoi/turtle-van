use poise::serenity_prelude::{ChannelId, GuildId};
use songbird::{Call, Songbird};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::super::error::MusicYoutubeError;
use super::get_user_voice_channel_id::get_user_voice_channel_id;
use crate::shared::{Context, Error};

pub struct VoiceContext {
    pub guild_id: GuildId,
    pub voice_client: Arc<Songbird>,
}

impl VoiceContext {
    pub async fn from_ctx(ctx: &Context<'_>) -> Result<Self, Error> {
        let guild_id = ctx.guild_id().ok_or(MusicYoutubeError::GuildIdNotFound)?;
        let voice_client = songbird::get(ctx.serenity_context())
            .await
            .ok_or(MusicYoutubeError::SongbirdClientNotFound)?
            .clone();
        Ok(Self {
            guild_id,
            voice_client,
        })
    }

    pub fn require_call(&self) -> Result<Arc<Mutex<Call>>, MusicYoutubeError> {
        self.voice_client
            .get(self.guild_id)
            .ok_or(MusicYoutubeError::BotNotInVoiceChannel)
    }

    pub fn require_user_channel(ctx: &Context<'_>) -> Result<ChannelId, MusicYoutubeError> {
        let guild = ctx.guild().ok_or(MusicYoutubeError::GuildNotFound)?;
        get_user_voice_channel_id(&guild.voice_states, ctx.author().id)
    }

    pub async fn validated_call(ctx: &Context<'_>) -> Result<Option<Arc<Mutex<Call>>>, Error> {
        if let Err(e) = Self::require_user_channel(ctx) {
            ctx.say(e.to_string()).await?;
            return Ok(None);
        }
        let vc = Self::from_ctx(ctx).await?;
        match vc.require_call() {
            Ok(call) => Ok(Some(call)),
            Err(e) => {
                ctx.say(e.to_string()).await?;
                Ok(None)
            }
        }
    }
}
