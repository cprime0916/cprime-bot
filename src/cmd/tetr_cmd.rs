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
    pub async fn player(ctx: Context<'_>, username: String) -> Result<(), Error>{
        ctx.say("# skibidi toilet\nMaking api fetch cmd").await?;
        let toml_info = fs::read_to_string("config.toml")?;
        let config_toml: Config = toml::from_str(&toml_info)?;
        let mut user_url = config_toml.tetr.url;
        user_url.push_str(username.as_ref());
        let response = reqwest::get(user_url).await?;
        println!("{:?}", &response.text().await?);
        Ok(())
    }
}