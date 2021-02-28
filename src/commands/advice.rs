use serde::Deserialize;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Deserialize)]
struct Slip {
    id: i32,
    advice: String,
}

#[derive(Deserialize)]
struct Advice {
    slip: Slip,
}

#[command]
#[description = "Asks for the advice of the gods and reveals their musings."]
#[usage = ""]
async fn advice(ctx: &Context, msg: &Message) -> CommandResult {
    const ENDPOINT: &str = "https://api.adviceslip.com/advice";
    let advice = reqwest::get(ENDPOINT).await?.json::<Advice>().await?;
    let results = format!("{} - #{}", advice.slip.advice, advice.slip.id);

    let _ = msg.channel_id.say(&ctx.http, results).await;
    Ok(())
}
