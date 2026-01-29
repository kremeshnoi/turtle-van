mod features;
mod shared;
use crate::shared::AppError;
use crate::shared::Data;
use crate::shared::*;
use dotenv::dotenv;
use features::music;

use poise::serenity_prelude;
use reqwest::Client as HttpClient;
use serenity::prelude::TypeMapKey;
use songbird::SerenityInit;

struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let discord_token = std::env::var("DISCORD_TOKEN")
        .unwrap_or_else(|_| panic!("{}", AppError::DiscordTokenNotFound));

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
            prefix: Some(CMD_PREFIX_SIGN.into()),
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
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .unwrap_or_else(|_| panic!("{}", AppError::ClientCreateFailed));

    if let Err(error) = client.start().await {
        println!("{}", AppError::ClientStartFailed(error.to_string()));
    }
}
