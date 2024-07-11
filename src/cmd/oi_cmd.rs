#![allow(dead_code, unused_variables, unreachable_code, unused_mut)]
#![allow(clippy::wildcard_imports)]

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
use tokio::time::{sleep, Duration};
use crate::config::Config;
use crate::utils::{traits::Cmd, deserializer::ContestInfo};
use crate::{Context, Data, Error, walao};
use crate::utils::constant::*;
use crate::utils::types::{Algorithms, ExpectError};

const DFS_CODE: &str = "void dfs(int u){\
                        \n  if(visited[u]) return;\
                        \n  visited[u] = 1;\
                        \n  for(auto& v : u) dfs(v);\
                        }";
const FIB_CODE: &str = "int fib(int n){\
                       \n   vector<int> fib(n+1);\
                       \n   fib[0] = 1;\
                       \n   fib[1] = 1;\
                       \n   for(int i=2;i<=n;i++) fib[i] = fib[i-1]+fib[i-2];\
                       \n   return fib[n];\
                       \n}";
const BINARY_SEARCH_CODE: &str = "bool binary_search(int n, int a[], int target){\
                                    \n  int low = 0;\
                                    \n  int high = n-1;\
                                    \n  while(low <= high){\
                                    \n      int mid = (low+high)/2;\
                                    \n      if(a[mid] < target) low = m+1;\
                                    \n      else if(a[mid] > target) high = m-1;\
                                    \n      else return true;\
                                    \n  }
                                    \n  return false;
                                    \n}";

type ContestTuple = (usize, String, String, DateTime<FixedOffset>, String, String);
type Field<'a> = (&'a String, String, bool);
pub(crate) struct OiCmd;

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

                        let start = format!("{start_utc8}");
                        let end = format!("{end_utc8}");

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

impl Cmd for OiCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::contests(), Self::explain()]
    }
}

impl OiCmd{
    fn construct_embed(current_page: i32, total_pages: i32, ctv: Vec<ContestTuple>) -> CreateEmbed{
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
            .color(NAVY_BLUE)
            .title(format!("Upcoming Contests (Page {}/{})", current_page+1, total_pages))
            .fields(fields);

        embed
    }
    
    fn algo_embed(algo: Algorithms) -> CreateEmbed{
        let mut title: String;
        let mut code: String;
        match algo{
            Algorithms::Dfs => {
                title = String::from("Depth-first search (DFS)");
                code = String::from(DFS_CODE);
                
            }
            Algorithms::DpFib => {
                title = String::from("Fibonacci w/ memoization");
                code = String::from(FIB_CODE);
            }
            Algorithms::BinSearch => {
                title = String::from("Binary search");
                code = String::from(BINARY_SEARCH_CODE);
            }
        }
        let code_fmt = format!("```cpp\n{}```", code);
        CreateEmbed::new().color(NAVY_BLUE).title(title).field("Implementation", code_fmt, false)
    }
    
    /// Command for obtaining contest data,
    /// you can choose a specific website to view recent contests too.
    #[poise::command(prefix_command, slash_command, category="OiCmd", aliases("contest", "ct"))]
    async fn contests(ctx: Context<'_>, host: Option<String>) -> Result<(), Error>{
        let mut contest_info: Vec<ContestTuple>;
        if let Some(ref s) = host{
            contest_info = get_contests(vec![host.clone().unwrap().as_ref()]).await?;
        } else {
            contest_info = get_contests(vec![
                "codeforces.com",
                "atcoder.jp",
                "codechef.com",
                "leetcode.com",
                "luogu.com.cn"
            ]).await?;
        }
        let total_pages = (contest_info.len() as i32+PAGE_LEN-1)/PAGE_LEN;
        let mut page = 0;
        let emoji_list = vec!['⬅', '➡'];
        let left_arrow = String::from("⬅");
        let right_arrow = String::from("➡");
        if contest_info.is_empty() {
            let message = CreateMessage::new()
                .embed(CreateEmbed::new()
                    .color(Colour::RED)
                    .title("WALAO!")
                    .description("Don't try to troll 💀"));
            let mut msg = ctx.channel_id().send_message(&ctx, message).await?;
        } else {
            let message = CreateMessage::new()
                .embed(OiCmd::construct_embed(page, total_pages, contest_info.clone()))
                .reactions(emoji_list.clone().into_iter());
            let mut msg = ctx.channel_id().send_message(&ctx, message).await?;

            let w: String = ExpectError::ReactionNotFound.into();
            sleep(Duration::from_millis(100)).await;

            while let Some(reaction) = msg.await_reaction(ctx).await {
                match reaction.emoji {
                    ReactionType::Unicode(ref emoji) if emoji == &right_arrow => {
                        
                        msg.delete_reaction(&ctx, reaction.user_id, emoji_list[1])
                            .await
                            .expect(walao!(expect, w).as_ref());
                        if page < total_pages - 1 {
                            page += 1;
                            let builder = EditMessage::new()
                                .embed(OiCmd::construct_embed(page, total_pages, contest_info.clone()));
                            msg.edit(ctx, builder).await?;
                        }
                    }
                    ReactionType::Unicode(ref emoji) if emoji == &left_arrow => {
                        msg.delete_reaction(&ctx, reaction.user_id, emoji_list[0])
                            .await
                            .expect(walao!(expect, w).as_ref());
                        if page > 0 {
                            page -= 1;
                            let builder = EditMessage::new()
                                .embed(OiCmd::construct_embed(page, total_pages, contest_info.clone()));
                            msg.edit(ctx, builder).await?;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    
    #[poise::command(slash_command, category="OiCmd")]
    async fn explain(ctx: Context<'_>, algo: Algorithms) -> Result<(), Error>{
        let msg = CreateMessage::new().embed(OiCmd::algo_embed(algo));
        let _ = ctx.channel_id().send_message(&ctx, msg).await?;
        Ok(())
    }
}