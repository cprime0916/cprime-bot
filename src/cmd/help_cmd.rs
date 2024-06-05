use poise::Command;
use crate::{Context, Data, Error};
use crate::utils::Cmd;

const DEFAULT_DESCRIPTION: &str = "No description given.";

pub(crate) struct HelpCmd;

impl Cmd for HelpCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::help(),]
    }
}

impl HelpCmd{
    /// Assisting you w/ this command
    #[poise::command(prefix_command, slash_command, category="HelpCmd")]
    pub async fn help(ctx: Context<'_>) -> Result<(), Error>{
        let commands = &ctx.framework().options().commands;
        let mut message = String::from("All commands:\n```");
        for command in commands{
            message.push_str(&format!("{}: {}\n", command.name, command.description
                .as_deref()
                .unwrap_or(DEFAULT_DESCRIPTION)));
        }
        message.push_str("\n```");
        ctx.say(message).await?;
        Ok(())
    }
}