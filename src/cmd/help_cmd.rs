use poise::Command;
use crate::{Context, Data, Error};
use crate::utils::Cmd;

pub(crate) struct HelpCmd;

impl Cmd for HelpCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::help(),]
    }
}

impl HelpCmd{
    /// Assisting you w/ this command
    #[poise::command(prefix_command, slash_command, category="HelpCmd")]
    pub async fn help(
        ctx: Context<'_>, #[description="Command you need help with"]
        command: Option<String>
    ) -> Result<(), Error>{
        let config = poise::builtins::HelpConfiguration {
                extra_text_at_bottom: "\
                Type .help <cmd> for more info on a command.
                You can edit your message to the bot and the bot will edit its response.",
                ..Default::default()
        };
        poise::builtins::help(ctx, command.as_deref(), config).await?;
        Ok(())
    }
}