use poise::Command;
use crate::{Data, Error};
#[macro_export]
macro_rules! delete_message {
    ($ctx:expr) => {
        if let Err(e) = $ctx.channel_id().delete_message(&$ctx, $ctx.id()).await {
            eprintln!("{}", e);
            $ctx.say("How to delete this fucking message?").await?;
        }
    };
}

pub trait Cmd{
    fn commands() -> Vec<Command<Data, Error>>;
}