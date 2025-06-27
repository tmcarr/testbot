use rand::seq::IteratorRandom;
use crate::{Context, Error};

/// Reply with a suggestion of fine beverage.
#[poise::command(slash_command, prefix_command)]
pub async fn drink(ctx: Context<'_>) -> Result<(), Error> {
    let responses = vec![
        "Water.",
        "Topo Chico",
        "La Croix",
        "Water?",
        "Milk",
        "Sparking Water",
        "Seltzer Water",
        "Tap Water",
        "Voda",
        "Dihydrogen monoxide",
        "Vand",
        "Eau",
        "Akvo",
        "Agua",
    ];

    let drink = responses.iter().choose(&mut rand::rng()).unwrap();

    ctx.say(*drink).await?;

    Ok(())
}
