use crate::{Context, Error};
use rand::thread_rng;
use rand::Rng;
use std::fs;
pub(crate) struct OtherCmd;
impl OtherCmd {
    #[poise::command(prefix_command)]
    pub async fn say(
        ctx: Context<'_>,
        args: Vec<String>,
    ) -> Result<(), Error> {
        let s = args.join(" ");
        match ctx.channel_id().delete_message(&ctx, ctx.id()).await {
            Ok(_) => {
                ctx.say(s).await?;
            }
            Err(e) => {
                eprintln!("{}", e);
                ctx.say("How to delete this fucking message?").await?;
            }
        }
        Ok(())
    }

    #[poise::command(prefix_command)]
    pub async fn quote(ctx: Context<'_>) -> Result<(), Error>{
        let rand = thread_rng().gen_range(1..7);
        match rand{
            1 => {
                ctx.say(":nerd:").await?;
            }
            2 => {
                let q = "\"I want, I like, I do!\"";
                let msg = format!("C' once said, {}", q);
                ctx.say(msg).await?;
            }
            3 => {
                ctx.say("random AC technique is the best technique").await?;
            }
            4 => {
                let q = "\"urmom\"";
                let msg = format!("Momo said, {}", q);
                ctx.say(msg).await?;
            }
            5 => {
                let q = "\"Less effort you made, more chance to <:ac:1171452943988428802>\"";
                let msg = format!("Kiu said, {}", q);
                ctx.say(msg).await?;
            }
            6 => {
                let q = "\"asfaf>>>>?????????ehzdvzdvzdv\"";
                let msg = format!("partialdiff said, {}", q);
                ctx.say(msg).await?;
            }
            _ => {
                ctx.say("Err it seems like I have a skill issue, I can't even quote :skull:").await?;
                eprintln!("How the fuck is it not in the range?");
            }
        }
        Ok(())
    }
}