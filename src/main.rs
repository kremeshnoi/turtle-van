mod cmd;
mod types;
mod consts;

use songbird::SerenityInit;
use poise::serenity_prelude;
use serenity::Client as SerenityClient;

use types::Data;

use cmd::{
    join,
    leave,
    play,
};

use crate::consts::{
    CMD_PREFIX_SIGN,
    FAILED_TO_CREATE_CLIENT,
    FAILED_TO_RETRIEVE_DISCORD_TOKEN,
    FAILED_TO_START_CLIENT,
};

#[tokio::main]
async fn main() {
    let discord_token = std::env::var("DISCORD_TOKEN").expect(FAILED_TO_RETRIEVE_DISCORD_TOKEN);
    let gateway_intents = serenity_prelude::GatewayIntents::non_privileged() | serenity_prelude::GatewayIntents::MESSAGE_CONTENT;
    let framework_options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(CMD_PREFIX_SIGN.into()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        commands: vec![
            join::join(),
            play::play(),
            leave::leave()
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(framework_options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = SerenityClient::builder(discord_token, gateway_intents)
        .framework(framework)
        .register_songbird()
        .await
        .expect(FAILED_TO_CREATE_CLIENT);

    if let Err(error) = client.start().await {
        println!("{}: {}", FAILED_TO_START_CLIENT, error);
    }
}