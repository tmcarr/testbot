use rand::seq::IteratorRandom;
use crate::{Context, Error};

/// Reply with a suggestion for cuisine.
#[poise::command(slash_command, prefix_command)]
pub async fn food(ctx: Context<'_>) -> Result<(), Error> {
    let responses = [
        "Asian", "Barbecue", "Burgers", "Italian", "Mexican", "Pho", "Pizza", "Steak", "Seafood",
        "Indian", "Cajun",
    ];

    let item = responses.iter().choose(&mut rand::rng()).unwrap();

    ctx.say(*item).await?;

    Ok(())
}
