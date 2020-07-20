use serde::Deserialize;
// use serde_json::{Result};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Deserialize, Debug)]
struct AdviceSlip {
    slip_id: u32,
    advice: String,
}

#[command]
fn advice(ctx: &mut Context, msg: &Message) -> CommandResult {
    let endpoint = "https://api.adviceslip.com/advice";
    let slip: AdviceSlip = reqwest::blocking::get(endpoint)?.json()?;
    let results = format!("{}: {}", slip.slip_id, slip.advice);

    let _ = msg.channel_id.say(&ctx.http, results);
    Ok(())
}

#[derive(Deserialize, Debug)]
struct AdviceSearch {
    total_results: u32,
    query: u32,
    slips: Vec<AdviceSlip>,
}

#[derive(Deserialize, Debug)]
struct AdviceMessage {
    r#type: String,
    text: String,
}

#[command]
fn advice_id(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    let _endpoint = "https://api.adviceslip.com/advice/{slip_id}";
    Ok(())
}

#[command]
fn advice_search(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    let _endpoint = "https://api.adviceslip.com/advice/search/{query}";
    Ok(())
}
