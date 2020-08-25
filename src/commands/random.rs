use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("rand")]
pub fn random(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let choices = args.raw().collect::<Vec<&str>>();

    if let Some(choice) = choices.choose(&mut rand::thread_rng()) {
      msg.channel_id.say(&ctx.http, choice)?;
    } else {
      msg.channel_id.say(&ctx.http, "Why u no args?!")?;
    }


    Ok(())
}
