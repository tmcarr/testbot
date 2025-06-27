use crate::{Context, Error};

/// Reply with a link to the bot's source code
#[poise::command(slash_command, prefix_command)]
pub async fn github(ctx: Context<'_>) -> Result<(), Error> {
    let github = "https://github.com/tmcarr/testbot";
    ctx.say(&format!("My code is at: {}", &github)).await?;
    Ok(())
}
