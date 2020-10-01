use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("rand")]
async fn random(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let choices = args.raw().collect::<Vec<&str>>();

    let thing = choices.choose(&mut rand::thread_rng());

    match thing {
        Some(choice) => msg.channel_id.say(&ctx.http, choice).await?,
        _ => msg.channel_id.say(&ctx.http, "Why u no args?!").await?,
    };

    Ok(())
}
