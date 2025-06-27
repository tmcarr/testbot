use crate::{Context, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct Slip {
    id: i32,
    advice: String,
}

#[derive(Deserialize)]
struct Advice {
    slip: Slip,
}

/// Asks for the advice of the gods and reveals their musings.
#[poise::command(slash_command, prefix_command)]
pub async fn advice(ctx: Context<'_>) -> Result<(), Error> {
    const ENDPOINT: &str = "https://api.adviceslip.com/advice";
    let advice = reqwest::get(ENDPOINT).await?.json::<Advice>().await?;
    let results = format!("{} - #{}", advice.slip.advice, advice.slip.id);

    ctx.say(results).await?;
    Ok(())
}
