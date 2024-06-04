#![allow(dead_code, unreachable_code, unused_variables)]

use poise::Command;
use crate::{Cmd, Context, Data, Error};

pub(crate) struct ContestCmd;

fn get_contests(hosts: Vec<String>) -> !{
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