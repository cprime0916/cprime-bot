use std::fs;
use poise::Command;
use serde_json as json;
use crate::{Context, Data, Error};
use crate::utils::traits::Cmd;
use toml;
use crate::config::Config;
use crate::utils::deserializer::TetrInfo;

pub(crate) struct TetrCmd;

impl Cmd for TetrCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::player(),]
    }
}

impl TetrCmd{
    #[poise::command(prefix_command)]
    async fn player(ctx: Context<'_>, username: String) -> Result<(), Error>{
        ctx.say("Making progress for implementing tetr command").await?;
        let toml_info = fs::read_to_string("config.toml")?;
        let config_toml: Config = toml::from_str(&toml_info)?;
        let mut user_url = config_toml.tetr.user_url;
        user_url.push_str(username.as_ref());
        let response = reqwest::get(user_url).await?;
        let text = response.text().await?;
        let info: TetrInfo = json::from_str(text.as_ref())?;
        println!("{info:#?}");
        Ok(())
    }
}