use std::fs;
use poise::Command;
use crate::{Context, Data, Error};
use crate::utils::traits::Cmd;
use toml;
use crate::config::Config;

pub(crate) struct TetrCmd;

impl Cmd for TetrCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::player(),]
    }
}

impl TetrCmd{
    #[poise::command(prefix_command)]
    async fn player(ctx: Context<'_>, username: String) -> Result<(), Error>{
        ctx.say("# Walao\nMaking progress for implementing tetr command").await?;
        let toml_info = fs::read_to_string("config.toml")?;
        let config_toml: Config = toml::from_str(&toml_info)?;
        let mut user_url = config_toml.tetr.user_url;
        user_url.push_str(username.as_ref());
        let response = reqwest::get(user_url).await?;
        let s = format!("{response:#?}");
        ctx.say(s).await?;
        Ok(())
    }
}