use serde::de;
use serde::{self, Deserialize, Deserializer};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fmt::Display;
use std::str::FromStr;

use crate::AlphaVantageAPIToken;

#[command]
#[aliases("stocks", "stock", "stonks", "stonk")]
async fn stonks(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    for stonk in args.iter::<String>() {
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                &format!(
                    "https://www.finviz.com/chart.ashx?t={}&ty=c&ta=1&p=d&s=l",
                    &stonk.unwrap()
                ),
            )
            .await;
    }
    Ok(())
}

#[command]
#[aliases("stockcomp", "stonkcomp", "s&pcomp")]
async fn stonkcomp(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    for stonk in args.iter::<String>() {
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                &format!("https://stonks.egd.pw/spcomp?symbol={}", &stonk.unwrap()),
            )
            .await;
    }
    Ok(())
}

// Example Response at https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Deserialize, Debug)]
struct Quote {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(deserialize_with = "from_str")]
    #[serde(rename = "02. open")]
    open: f32,
    #[serde(deserialize_with = "from_str")]
    #[serde(rename = "03. high")]
    high: f32,
    #[serde(deserialize_with = "from_str")]
    #[serde(rename = "04. low")]
    low: f32,
    #[serde(deserialize_with = "from_str")]
    #[serde(rename = "05. price")]
    price: f32,
    #[serde(deserialize_with = "from_str")]
    #[serde(rename = "06. volume")]
    volume: u64,
    // TODO: Handle datestrings to render better...
    #[serde(rename = "07. latest trading day")]
    latest_day: String,
    #[serde(deserialize_with = "from_str")]
    #[serde(rename = "08. previous close")]
    prev_close: f32,
    #[serde(deserialize_with = "from_str")]
    #[serde(rename = "09. change")]
    change: f32,
    // TODO: Handle the % within this field....
    #[serde(rename = "10. change percent")]
    change_pct: String,
}

#[derive(Deserialize)]
struct GlobalQuote {
    #[serde(rename = "Global Quote")]
    quote: Quote,
}

#[command]
#[aliases("sprice", "stonkprice", "stockprice")]
async fn price(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Read our API token out of the ctx
    let data_read = ctx.data.read().await;
    let api_token = data_read
        .get::<AlphaVantageAPIToken>()
        .expect("Expected an AlphaVantage API token in the context.");

    let ticker = args.single::<String>().unwrap();
    let endpoint = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        ticker, api_token
    );
    let globalquote = reqwest::get(&endpoint).await?.json::<GlobalQuote>().await?;

    // TODO: Generalize this so I dont have to repeat myself constantly....
    // TODO: Come up with a method of tackng on another N columns here so I can do comparisons....
    let message = MessageBuilder::new()
        .quote_rest()
        .push_bold_line(globalquote.quote.symbol)
        .push_mono_line(format!("{:<15}{:<10}", "Price:", globalquote.quote.price))
        .push_mono_line(format!("{:<15}{:<10}", "Open:", globalquote.quote.open))
        .push_mono_line(format!("{:<15}{:<10}", "High:", globalquote.quote.high))
        .push_mono_line(format!("{:<15}{:<10}", "Low:", globalquote.quote.low))
        .push_mono_line(format!("{:<15}{:<10}", "Change:", globalquote.quote.change))
        .push_mono_line(format!(
            "{:<15}{:<10}",
            "% Change:", globalquote.quote.change_pct
        ))
        .push_mono_line(format!(
            "{:<15}{:<10}",
            "Prev. Close:", globalquote.quote.prev_close
        ))
        .push_mono_line(format!("{:<15}{:<10}", "Volume:", globalquote.quote.volume))
        .push_mono_line(format!(
            "{:<15}{:<10}",
            "Latest Day:", globalquote.quote.latest_day
        ))
        .build();

    let _ = msg.channel_id.say(&ctx.http, message).await?;

    Ok(())
}
