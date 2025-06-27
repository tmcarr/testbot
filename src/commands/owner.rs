use crate::{Context, Error};

/// Causes the bot to die.
#[poise::command(prefix_command, owners_only)]
pub async fn quit(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Shutting down!").await?;

    // For now, just send a message. The bot can be stopped externally.
    // TODO: Implement proper shutdown mechanism

    Ok(())
}
