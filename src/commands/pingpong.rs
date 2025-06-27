use crate::{Context, Error};

/// Reply with 'Pong!'
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// Let er' rip!
#[poise::command(slash_command, prefix_command)]
pub async fn fart(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Thbbbbbbbbbbbbbbt.... squeak.").await?;
    Ok(())
}
