use std::fs;
use poise::Command;
use serde_json as json;
use serenity::all::{Colour, CreateEmbed, CreateMessage};
use crate::{Context, Data, Error};
use crate::utils::traits::Cmd;
use toml;
use crate::config::Config;
use crate::utils::deserializer::{TetrInfo, TetrUser};

pub(crate) struct TetrCmd;
type Field<'a> = (&'a str, String, bool);

impl Cmd for TetrCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::player(),]
    }
}

impl TetrCmd{
    fn construct_embed(user_info: TetrUser) -> CreateEmbed {
        let fields = vec![
            ("Username", user_info.username(), false),
            ("Role", user_info.role(), false),
            ("Time created", user_info.timestamp(), false),
        ];
        
        let embed = CreateEmbed::new()
            .color(Colour::ROHRKATZE_BLUE)
            .title("User Info")
            .fields(fields);
        
        embed
    }
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
        
        let message = CreateMessage::new().embed(TetrCmd::construct_embed(info.user().unwrap()));
        let _ = ctx.channel_id().send_message(&ctx, message).await?;
        Ok(())
    }
}