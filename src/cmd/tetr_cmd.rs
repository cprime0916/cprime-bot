use std::fs;
use chrono::{FixedOffset, NaiveDateTime};
use poise::Command;
use serde_json as json;
use serenity::all::{Colour, CreateEmbed, CreateMessage};
use crate::{Context, Data, Error, walao};
use crate::utils::traits::Cmd;
use toml;
use crate::config::Config;
use crate::utils::deserializer::{TetrInfo, TetrUser};
use crate::utils::constant::*;

pub(crate) struct TetrCmd;
type Field<'a> = (&'a str, String, bool);

impl Cmd for TetrCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::player(),]
    }
}

impl TetrCmd{
    fn construct_embed(user_info: TetrUser) -> Result<CreateEmbed, Error> {
        let timestamp_utc8 = NaiveDateTime::parse_from_str(user_info.timestamp().as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ")?
            .and_utc()
            .with_timezone(&FixedOffset::east_opt(UTC8).unwrap())
            .format("%d-%m-%Y %H:%M:%S");
        let ts_str = format!("{timestamp_utc8}");
        let role = match user_info.role().chars().next() {
            Some(first_char) => first_char.to_uppercase().chain(user_info.role().chars().skip(1)).collect::<String>(),
            None => String::new(),
        };
        let minutes = (user_info.stats().3/60.0).round() as isize;
        let stats = format!("XP: {}\nWin/Played: {}/{}\nTotal playing time: {} minutes\n", user_info.stats().0, user_info.stats().2, user_info.stats().1, minutes);
        let mut badge_str = String::new();
        if let Some(b) = user_info.get_badges(){
            for badge in b{
                badge_str.push_str(format!("{badge}").as_str());
            }
        }
        
        let mut fields: Vec<Field> = vec![
            ("Username", user_info.username(), true),
            ("Country/Region", user_info.country().1, true),
            ("Role", role, false),
            ("Time created", ts_str, false),
            ("Stats", stats, false),
        ];
        
        if !badge_str.is_empty(){
            fields.push(("Badges", badge_str, false));
        }
        
        if let Some(i) = user_info.bot_info(){
            fields.push(("Bot Info", i, false));
        }
        
        let embed = CreateEmbed::new()
            .color(NAVY_BLUE)
            .title("User Info")
            .fields(fields);
        
        Ok(embed)
    }
    #[poise::command(prefix_command, category="TetrCmd")]
    async fn player(ctx: Context<'_>, username: String) -> Result<(), Error>{
        let toml_info = fs::read_to_string("config.toml")?;
        let config_toml: Config = toml::from_str(&toml_info)?;
        let mut user_url = config_toml.tetr.user_url;
        user_url.push_str(username.to_ascii_lowercase().as_ref());
        
        let response = reqwest::get(user_url).await?;
        let text = response.text().await?;
        println!("{text}");
        let info: TetrInfo = json::from_str(text.as_ref())?;
        let mut message = CreateMessage::new();
        match info.user(){
            Some(i) => {
                message = message.embed(TetrCmd::construct_embed(i).expect(walao!(expect, "Invalid parsing").as_ref()));
            }
            None => {
                message = message.embed(CreateEmbed::new()
                    .color(Colour::RED)
                    .title("WALAO!")
                    .description("Don't try to troll 💀"));
            }
        }
        let _ = ctx.channel_id().send_message(&ctx, message).await?;
        Ok(())
    }
}