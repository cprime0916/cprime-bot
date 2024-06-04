use poise::Command;
use crate::{Cmd, Data, Error};

pub(crate) struct ContestCmd;
impl Cmd for ContestCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        todo!("Put all commands into this function");
    }
}

impl ContestCmd{
    todo!("implement contest functionality");
}