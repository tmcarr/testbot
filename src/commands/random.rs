use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::seq::SliceRandom;

#[command]
#[aliases("rand")]
pub fn random(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let choices = args.raw().collect::<Vec<&str>>();

    let _ = msg.channel_id.say(
        &ctx.http,
        choices.choose(&mut rand::thread_rng()).unwrap(),
    );

    Ok(())
}