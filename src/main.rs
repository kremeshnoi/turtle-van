mod cmd;
mod consts;
mod types;

use poise::serenity_prelude;
use reqwest::Client as HttpClient;
use serenity::prelude::TypeMapKey;
use songbird::SerenityInit;

use cmd::{join, leave, play};
use crate::types::Data;

use crate::consts::{
    CMD_PREFIX_SIGN, FAILED_TO_CREATE_CLIENT, FAILED_TO_RETRIEVE_DISCORD_TOKEN,
    FAILED_TO_START_CLIENT,
};

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[tokio::main]
async fn main() {
    let discord_token = std::env::var("DISCORD_TOKEN").expect(FAILED_TO_RETRIEVE_DISCORD_TOKEN);
    let gateway_intents = serenity_prelude::GatewayIntents::non_privileged()
        | serenity_prelude::GatewayIntents::MESSAGE_CONTENT;
    let framework_options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(CMD_PREFIX_SIGN.into()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        commands: vec![join::join(), play::play(), leave::leave()],
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

    let mut client = serenity_prelude::ClientBuilder::new(discord_token, gateway_intents)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect(FAILED_TO_CREATE_CLIENT);

    if let Err(error) = client.start().await {
        println!("{}: {}", FAILED_TO_START_CLIENT, error);
    }
}
