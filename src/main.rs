mod features;
mod shared;

use anyhow::Context;
use dotenv::dotenv;
use features::music;
use poise::serenity_prelude;
use shared::Data;
use songbird::SerenityInit;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let discord_token =
        std::env::var("DISCORD_TOKEN").context("Failed to retrieve DISCORD_TOKEN")?;

    let all_commands = {
        let mut cmds = Vec::new();
        cmds.extend(music::commands());
        cmds
    };

    let gateway_intents = serenity_prelude::GatewayIntents::non_privileged()
        | serenity_prelude::GatewayIntents::GUILDS
        | serenity_prelude::GatewayIntents::GUILD_MESSAGES
        | serenity_prelude::GatewayIntents::GUILD_VOICE_STATES
        | serenity_prelude::GatewayIntents::MESSAGE_CONTENT;

    let framework_options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("van!".into()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        commands: all_commands,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(framework_options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::new())
            })
        })
        .build();

    let mut client = serenity_prelude::ClientBuilder::new(discord_token, gateway_intents)
        .framework(framework)
        .register_songbird()
        .await
        .context("Failed to create client")?;

    client.start().await.context("Failed to start client")?;

    Ok(())
}
