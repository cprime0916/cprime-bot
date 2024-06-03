#![allow(unreachable_code, unused_variables)]
use crate::{Context, Data, delete_message, Error, error_print};
use rand::thread_rng;
use rand::Rng;
use poise::Command;
use crate::utils::Cmd;

const LEFT_PRAYER: &str = "O Left,\n\
                            Lord of AK,\
                            \nwhom leads to the truthful and righteous path.\
                            \nmay thou grant us the way to AK,\
                            \nthe path with AC'ing tasks in thy name,\
                            \nMay we work as we worship you.\
                            \nIn the name of the mythical and legendary Left.\
                            \nAccepted.";
const CODE_PRAYER: &str = "O Left,\
                            \nLord of AK,\
                            \nmay thou grant us the intelligence, the wisdom,\
                            \nto write elegant code & programs in thy name.\
                            \nMay thou guide us to elegant approaches,\
                            \nto increase ranking in thy name,\
                            \n& under thy glory.\
                            \nAccepted.";

pub(crate) struct OtherCmd;
impl Cmd for OtherCmd{
    fn commands() -> Vec<Command<Data, Error>> {
        vec![Self::say(), Self::quote(), Self::agree(), Self::disagree(), Self::reply(), Self::spam(),]
    }
}
impl OtherCmd {
    #[poise::command(prefix_command)]
    pub async fn say(
        ctx: Context<'_>,
        args: Vec<String>,
    ) -> Result<(), Error> {
        let s = args.join(" ");
        delete_message!(ctx);
        match args[0].as_ref() {
            "`-p`" => {
                ctx.say(LEFT_PRAYER).await?;
            }
            "`-cp`" => {
                ctx.say(CODE_PRAYER).await?;
            }
            _ => {
                ctx.say(s).await?;
            }
        }
        Ok(())
    }

    #[poise::command(prefix_command)]
    pub async fn quote(ctx: Context<'_>) -> Result<(), Error>{
        delete_message!(ctx);
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
                error_print!(ctx, "quote", "How the fuck is it not in the range?");
            }
        }
        Ok(())
    }

    #[poise::command(prefix_command)]
    pub async fn agree(ctx: Context<'_>) -> Result<(), Error>{
        delete_message!(ctx);
        let rand = thread_rng().gen_range(1..6);
        match rand{
            1 => {
                ctx.say("Indeed").await?;
            }
            2 => {
                ctx.say("fr").await?;
            }
            3 => {
                ctx.say("yeah").await?;
            }
            4 => {
                ctx.say("fax").await?;
            }
            5 => {
                ctx.say("Real").await?;
            }
            _ => {
                error_print!(ctx, "agree", "How the fuck is it not in the range?");
            }
        }
        Ok(())
    }

    #[poise::command(prefix_command)]
    pub async fn disagree(ctx: Context<'_>) -> Result<(), Error>{
        delete_message!(ctx);
        let rand = thread_rng().gen_range(1..7);
        match rand{
            1 => {
                ctx.say("nuh uh").await?;
            }
            2 => {
                ctx.say("naw").await?;
            }
            3 => {
                ctx.say("not real bruv").await?;
            }
            4 => {
                ctx.say("hell nah bruv this ain't it").await?;
            }
            5 => {
                ctx.say("you nigger").await?;
            }
            6 => {
                ctx.say("no u").await?;
            }
            _ => {
                error_print!(ctx, "disagree", "How the fuck is it not in the range?");
            }
        }
        Ok(())
    }

    #[poise::command(prefix_command)]
    pub async fn reply(ctx: Context<'_>) -> Result<(), Error>{
        todo!("Implement reply command");
        Ok(())
    }

    #[poise::command(prefix_command, user_cooldown=5)]
    pub async fn spam(ctx: Context<'_>, args: Vec<String>) -> Result<(), Error>{
        delete_message!(ctx);
        let spam_times = args.last()
            .unwrap()
            .parse::<u32>()
            .expect("Parsing uint32 error");
        let s = &args[0..args.len()-1].join(" ");
        let mut counter: u32 = 0;
        while counter != spam_times {
            ctx.say(s).await?;
            counter = counter + 1;
        }
        Ok(())
    }
}