use poise::Command;
use crate::{Data, Error};

pub trait Cmd{
    fn commands() -> Vec<Command<Data, Error>>;
}