use serde::Deserialize;
use serenity::framework::standard::{macros::command, Args, CommandResult};
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
async fn advice(ctx: &Context, msg: &Message) -> CommandResult {
    let endpoint = "https://api.adviceslip.com/advice";
    let result_text = reqwest::get(endpoint).await?.json::<Advice>().await?;
    let results = format!("{} - #{}", result_text.slip.advice, result_text.slip.id);

    let _ = msg.channel_id.say(&ctx.http, results).await;
    Ok(())
}
