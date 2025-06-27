use rand::seq::IteratorRandom;
use crate::{Context, Error};

/// A bot's gotta eat....
#[poise::command(slash_command, prefix_command)]
pub async fn botsnack(ctx: Context<'_>) -> Result<(), Error> {
    let responses = ["Yum!", "*cronch*", "MOAR", "*Smiles*", "Nice."];
    let response = responses.iter().choose(&mut rand::rng()).unwrap();

    ctx.say(*response).await?;

    Ok(())
}
