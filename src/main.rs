pub mod cmd;
pub mod config;
pub mod utils;

use poise::{Command, serenity_prelude as serenity};
use toml;
use std::fs;
use serenity::gateway::ActivityData;
use cmd::{other_cmd::OtherCmd, help_cmd::HelpCmd, contest_cmd::ContestCmd};
use cmd::tetr_cmd::TetrCmd;
use utils::traits::Cmd;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn commands() -> Vec<Command<Data, Error>>{
    let mut commands = OtherCmd::commands();
    commands.extend(HelpCmd::commands());
    commands.extend(ContestCmd::commands());
    commands.extend(TetrCmd::commands());
    commands
}

#[tokio::main]
async fn main() {
    let toml_info = fs::read_to_string("config.toml").unwrap();
    let config_toml: config::Config = toml::from_str(&toml_info).unwrap();
    let token = config_toml.discord.token;
    let intents = serenity::GatewayIntents::all();
    let activity = ActivityData::listening("Preußenlied");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
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