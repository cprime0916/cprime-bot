use std::fs;

use poise::serenity_prelude as serenity;
use serenity::gateway::ActivityData;

use cmd::{help_cmd::HelpCmd, oi_cmd::OiCmd, other_cmd::OtherCmd};
use cmd::structs::CmdBuilder;
use cmd::tetr_cmd::TetrCmd;

mod config;
mod utils;
mod cmd;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let toml_info = fs::read_to_string("config.toml").unwrap();
    let config_toml: config::Config = toml::from_str(&toml_info).unwrap();
    let token = config_toml.discord.token;
    let intents = serenity::GatewayIntents::all();
    let activity = ActivityData::listening("Preußenlied");
    let commands = CmdBuilder::new()
        .add_cmd::<OtherCmd>()
        .add_cmd::<HelpCmd>()
        .add_cmd::<OiCmd>()
        .add_cmd::<TetrCmd>()
        .build();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".to_owned()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .activity(activity)
        .await;
    client.unwrap().start().await.unwrap();
}