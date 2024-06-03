use poise::Command;
use crate::{Data, Error};

pub trait Cmd{
    fn commands() -> Vec<Command<Data, Error>>;
}

#[macro_export]
macro_rules! delete_message {
    ($ctx:expr) => {
        if let Err(e) = $ctx.channel_id().delete_message(&$ctx, $ctx.id()).await {
            eprintln!("{}", e);
            $ctx.say("How to delete this fucking message?").await?;
        }
    };
}

#[macro_export]
macro_rules! error_print {
    ($ctx:expr, $disability:expr, $error_output:expr) => {
        let error_message = concat!("Err it seems like I have a skill issue, I can't even ", $disability, " :skull:");
        $ctx.say(error_message).await?;
        eprintln!($error_output);
    };
}