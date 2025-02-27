use rand::prelude::IteratorRandom;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("rand")]
#[description = "Choose a random item from the list of inputs"]
#[usage = "foo bar baz"]
async fn random(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let choices = args.raw().collect::<Vec<&str>>();

    let thing = choices.iter().choose(&mut rand::rng());

    match thing {
        Some(choice) => msg.channel_id.say(&ctx.http, choice).await?,
        _ => msg.channel_id.say(&ctx.http, "Why u no args?!").await?,
    };

    Ok(())
}
