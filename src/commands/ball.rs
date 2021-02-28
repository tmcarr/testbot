use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("8ball")]
#[description = "Shakes the digital 8-ball."]
#[usage = ""]
async fn ball(ctx: &Context, msg: &Message) -> CommandResult {
    let responses = vec![
        "As I see it, yes.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don’t count on it.",
        "It is certain.",
        "It is decidedly so.",
        "Most likely.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Outlook good.",
        "Reply hazy, try again.",
        "Signs point to yes.",
        "Very doubtful.",
        "Without a doubt.",
        "Yes.",
        "Yes – definitely.",
        "You may rely on it.",
    ];

    let choice = responses.choose(&mut rand::thread_rng()).unwrap();

    let _ = msg.channel_id.say(&ctx.http, choice).await;

    Ok(())
}
