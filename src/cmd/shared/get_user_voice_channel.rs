use crate::types::{Context};
use poise::serenity_prelude::ChannelId;

pub fn get_user_voice_channel(ctx: &Context<'_>) -> Option<ChannelId> {
    ctx.guild()?.voice_states.get(&ctx.author().id)?.channel_id
}