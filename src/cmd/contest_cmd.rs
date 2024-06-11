#![allow(dead_code, unused_variables, unreachable_code)]

use toml;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json as json;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use std::fs;
use poise::Command;
use crate::config::Config;
use crate::utils::{Cmd, ContestInfo};
use crate::{Context, Data, Error, walao};

const UTC8: i32 = 3600 * 8;
const PAGE_LEN: i32 = 5;


type ContestTuple = (usize, String, String, DateTime<FixedOffset>, String, String);
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
                        let host = contest_data.host.as_deref().unwrap();
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

                        contests.push((duration, event.to_string(), host.to_string(), start_utc8_sort, start, end));
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
    fn construct_embed(ctv: Vec<ContestTuple>) -> !{

        todo!("Construct embed");
    }
    #[poise::command(prefix_command)]
    pub async fn contests(ctx: Context<'_>) -> Result<(), Error>{
        ctx.say("cprime still developing").await?;
        let contest_info = get_contests(vec!["codeforces.com", "atcoder.jp", "codechef.com", "leetcode.com",]).await?;
        let total_pages = (contest_info.len() as i32+PAGE_LEN-1)/PAGE_LEN;
        todo!("Implement embed message logic here");
        Ok(())
    }
}