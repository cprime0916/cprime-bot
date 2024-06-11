use poise::Command;
use serde::Deserialize;
use crate::{Data, Error};

pub trait Cmd{
    fn commands() -> Vec<Command<Data, Error>>;
}

#[macro_export]
macro_rules! delete_message {
    ($ctx:expr) => {
        if let Err(e) = $ctx.channel_id().delete_message(&$ctx, $ctx.id()).await {
            eprintln!("{}", e);
            $ctx.say("How to delete this fucking message?").await?;
        }
    };
}

/// # Walao macro
/// ## Overview
/// The walao macro is used as the debug print macro for me to find issues more easily.
/// It prints and sends the messages depending on the different arguments
/// ## Usage
/// when parsing json error: `walao!(parse, json);`
///
/// other use cases can be seen in the code below.
/// If the error is not defined, the `_` symbol will be used instead, like this
/// ```
/// walao!(_);
/// ```
#[macro_export]
macro_rules! walao{
    (status_code_error) =>  {
        eprintln!("Walao! Status code error, skill issue one");
    };
    (parse, json) => {
        eprintln!("Walao! Can't even parse json files! Skill issue one");
    };
    (parse, toml) => {
        eprintln!("Walao! Can't even parse toml files! Skill issue one");
    };
    ($ctx:expr, parse_config) => {
        let toml_info = fs::read_to_string("config.toml").unwrap();
        let config_toml: config::Config = toml::from_str(&toml_info).unwrap();
        let error_message = format!("# Walao!\n<@{}> skill issue one lah! I can't even parse the configuration", config_toml.discord.user_id);
        $ctx.say(error_message).await?;
        eprintln!("Walao! Parse configuration error");
    };
    ($ctx:expr, not_in_range) => {
        let toml_info = fs::read_to_string("config.toml").unwrap();
        let config_toml: config::Config = toml::from_str(&toml_info).unwrap();
        eprintln!("Walao! Not in range...");
        let error_message = format!("# Walao!\n<@{}> skill issue one lah! Not even in range", config_toml.discord.user_id);
        $ctx.say(error_message).await?;
    };
    (expect, parse) => {
        "Walao! Parsing error"
    };
    (_) => {
        eprintln!("Walao! This error siao one")
    };
}

#[derive(Deserialize)]
pub struct ContestInfo{
    pub meta: Option<Meta>,
    pub objects: Option<Vec<Contest>>,
}

#[derive(Deserialize)]
pub struct Meta{
    pub limit: Option<usize>,
    pub next: Option<String>,
    pub offset: Option<usize>,
    pub previous: Option<String>,
    pub total_count: Option<usize>,
}

#[derive(Deserialize)]
pub struct Contest{
    pub id: Option<usize>,
    pub resource: Option<String>,
    pub resource_id: Option<usize>,
    pub host: Option<String>,
    pub event: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub n_statistics: Option<usize>,
    pub n_problems: Option<usize>,
    pub parsed_at: Option<String>,
    pub duration: Option<usize>,
    pub href: Option<String>,
    pub problems: Option<String>,
}