use rand::prelude::IteratorRandom;
use crate::{Context, Error};

/// Choose a random item from the list of inputs
#[poise::command(slash_command, prefix_command)]
pub async fn random(
    ctx: Context<'_>,
    #[description = "Items to choose from"] items: String,
) -> Result<(), Error> {
    let choices: Vec<&str> = items.split_whitespace().collect();

    let thing = choices.iter().choose(&mut rand::rng());

    match thing {
        Some(choice) => ctx.say(*choice).await?,
        _ => ctx.say("Why u no args?!").await?,
    };

    Ok(())
}
