use crate::{Context, Error};
use rand::seq::IteratorRandom;

/// A bot's gotta eat....
#[poise::command(slash_command, prefix_command)]
pub async fn botsnack(ctx: Context<'_>) -> Result<(), Error> {
    let responses = ["Yum!", "*cronch*", "MOAR", "*Smiles*", "Nice."];
    let response = responses.iter().choose(&mut rand::rng()).unwrap();

    ctx.say(*response).await?;

    Ok(())
}
