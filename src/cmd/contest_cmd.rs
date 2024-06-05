#![allow(dead_code, unreachable_code, unused_variables, unused_imports)]

use std::fs;
use toml;
use reqwest::{Client, header};
use poise::Command;
use http::{Method, StatusCode};
use http::status::InvalidStatusCode;
use crate::{Cmd, config, Context, Data, Error, error_print};

pub(crate) struct ContestCmd;


async fn get_contests(hosts: Vec<String>) -> !{
    let toml_info = fs::read_to_string("config.toml").unwrap();
    let config_toml: config::Config = toml::from_str(&toml_info).unwrap();
    let mut query_vec = vec![("username", config_toml.clist.username),
                             ("api_key", config_toml.clist.key)
                             , ("upcoming", "true".to_owned())];
    for host in hosts{
        query_vec.push(("host", host));
        let response = Client::builder()
            .build()
            .unwrap()
            .get(&config_toml.clist.url)
            .query(&query_vec)
            .send()
            .await?;
        let data = response.json().await?;
        match response.status(){
            StatusCode::OK => {
                todo!("Implement adding contest info to a container");
            }
            _ => {
                todo!("Settle StatusCode Err");
            }
        }
    }
    todo!("Implement fetching & returning contest data as a Vec.");
}

impl Cmd for ContestCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![]
    }
}

impl ContestCmd{
    #[poise::command(prefix_command, slash_command)]
    pub async fn contests(ctx: Context<'_>) -> Result<(), Error>{
        todo!("Implement contest command");
        Ok(())
    }
}