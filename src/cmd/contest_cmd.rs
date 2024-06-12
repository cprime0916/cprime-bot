#![allow(dead_code, unused_variables, unreachable_code)]

use std::cmp::min;
use toml;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json as json;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use std::fs;
use poise::Command;
use serenity::all::{CreateEmbed, CreateMessage, EditMessage, ReactionType};
use serenity::model::Colour;
use crate::config::Config;
use crate::utils::{Cmd, ContestInfo};
use crate::{Context, Data, Error, walao};

const UTC8: i32 = 3600 * 8;
const PAGE_LEN: i32 = 5;

type ContestTuple = (usize, String, String, DateTime<FixedOffset>, String, String);
type Field<'a> = (&'a String, String, bool);
pub(crate) struct ContestCmd;

async fn get_contests(hosts: Vec<&str>) -> Result<Vec<ContestTuple>, Error> {
    let toml_info = fs::read_to_string("config.toml")?;
    let config_toml: Config = toml::from_str(&toml_info)?;

    let mut query_vec = vec![
        ("username", config_toml.clist.username.clone()),
        ("api_key", config_toml.clist.key.clone()),
        ("upcoming", "true".to_owned()),
    ];

    let mut contests = Vec::<ContestTuple>::new();

    for host in hosts {
        query_vec.push(("host", host.to_owned()));

        let response = Client::builder()
            .build()?
            .get(&config_toml.clist.url)
            .query(&query_vec)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        let data: ContestInfo = json::from_str(&text)?;

        match status {
            StatusCode::OK => {
                if let Some(contest_objects) = data.objects {
                    for contest_data in contest_objects {
                        let duration = contest_data.duration.unwrap_or(0) / 60;
                        let event = contest_data.event.as_deref().unwrap();
                        let href = contest_data.href.as_deref().unwrap();
                        let start_str = contest_data.start.as_deref().unwrap();
                        let end_str = contest_data.end.as_deref().unwrap();

                        let start_utc8_sort = NaiveDateTime::parse_from_str(start_str, "%Y-%m-%dT%H:%M:%S")?
                            .and_utc()
                            .with_timezone(&FixedOffset::east_opt(UTC8).unwrap());
                        let start_utc8 = NaiveDateTime::parse_from_str(start_str, "%Y-%m-%dT%H:%M:%S")?
                            .and_utc()
                            .with_timezone(&FixedOffset::east_opt(UTC8).unwrap())
                            .format("%d-%m-%Y %H:%M:%S");
                        let end_utc8 = NaiveDateTime::parse_from_str(end_str, "%Y-%m-%dT%H:%M:%S")?
                            .and_utc()
                            .with_timezone(&FixedOffset::east_opt(UTC8).unwrap())
                            .format("%d-%m-%Y %H:%M:%S");

                        let start = format!("{}", start_utc8);
                        let end = format!("{}", end_utc8);

                        contests.push((duration, event.to_string(), href.to_string(), start_utc8_sort, start, end));
                    }
                    contests.sort_by_key(|contest| contest.3);
                }
            }
            _ => {
                walao!(status_code_error);
            }
        }
    }

    Ok(contests)
}

impl Cmd for ContestCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::contests(),]
    }
}

impl ContestCmd{
    pub fn construct_embed(current_page: i32, ctv: Vec<ContestTuple>) -> CreateEmbed{
        let start_index = current_page*PAGE_LEN;
        let end_index = min((current_page+1)*PAGE_LEN, ctv.len() as i32);

        let mut fields: Vec<Field> = Vec::new();
        for index in start_index..end_index{
            let contest = &ctv[index as usize];
            let value = format!("Start Time: {}\n\
            End Time: {}\n\
            [Contest Link]({})", contest.4, contest.5, contest.2);
            fields.push((&contest.1, value,  false));
        }

        let embed = CreateEmbed::new()
            .color(Colour::BLITZ_BLUE)
            .title(format!("Upcoming Contests (Page {}/{})", current_page+1, PAGE_LEN))
            .fields(fields.into_iter());
        embed
    }
    #[poise::command(prefix_command, slash_command)]
    pub async fn contests(ctx: Context<'_>) -> Result<(), Error>{
        let contest_info = get_contests(vec![
            "codeforces.com",
            "atcoder.jp",
            "codechef.com",
            "leetcode.com",
            "luogu.com.cn"
        ]).await?;
        let total_pages = (contest_info.len() as i32+PAGE_LEN-1)/PAGE_LEN;
        let mut page = 0;
        let emoji_list = vec!['⬅', '➡'];
        let left_arrow = String::from("⬅");
        let right_arrow = String::from("➡");

        let message = CreateMessage::new()
            .embed(ContestCmd::construct_embed(page, contest_info.clone()))
            .reactions(emoji_list.clone().into_iter());
        let mut msg = ctx.channel_id().send_message(&ctx, message).await?;

        loop {
            match msg.await_reaction(&ctx).await {
                Some(reaction) => match reaction.emoji {
                    ReactionType::Unicode(ref emoji) if emoji == &right_arrow => {
                        msg.delete_reaction(&ctx, reaction.user_id, emoji_list[1])
                            .await
                            .expect(walao!(expect, find_react));
                        if page < total_pages - 1 {
                            page += 1;
                            let builder = EditMessage::new()
                                .embed(ContestCmd::construct_embed(page, contest_info.clone()));
                            msg.edit(ctx, builder).await?;
                        }
                    }
                    ReactionType::Unicode(ref emoji) if emoji == &left_arrow => {
                        msg.delete_reaction(&ctx, reaction.user_id, emoji_list[0])
                            .await
                            .expect(walao!(expect, find_react));
                        if page > 0 {
                            page -= 1;
                            let builder = EditMessage::new()
                                .embed(ContestCmd::construct_embed(page, contest_info.clone()));
                            msg.edit(ctx, builder).await?;
                        }
                    }
                    _ => {}
                },
                None => {
                    break;
                }
            }
        }
        Ok(())
    }
}