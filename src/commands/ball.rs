use rand::prelude::IteratorRandom;
use crate::{Context, Error};

/// Shakes the digital 8-ball.
#[poise::command(slash_command, prefix_command)]
pub async fn ball(ctx: Context<'_>) -> Result<(), Error> {
    let responses = vec![
        "As I see it, yes.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
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

    let choice = responses.iter().choose(&mut rand::rng()).unwrap();
    ctx.say(*choice).await?;

    Ok(())
}
