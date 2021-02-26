use serde::Deserialize;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

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

// Example Response
//
// {
//     "Global Quote": {
//         "01. symbol": "IBM",
//         "02. open": "123.3700",
//         "03. high": "124.3500",
//         "04. low": "122.3350",
//         "05. price": "122.4700",
//         "06. volume": "5672671",
//         "07. latest trading day": "2021-02-25",
//         "08. previous close": "123.2100",
//         "09. change": "-0.7400",
//         "10. change percent": "-0.6006%"
//     }
// }

#[derive(Deserialize)]
struct Quote {
    // #[serde(rename="01. symbol")]
    // symbol: String,
    // #[serde(rename="02. open")]
    // open: i32,
    // #[serde(rename="03. high")]
    // hight: i32,
    // #[serde(rename="04. low")]
    // low: i32,
    #[serde(rename = "05. price")]
    price: String,
    // #[serde(rename="06. volume")]
    // volume: u64,
    // #[serde(rename="07. latest trading day")]
    // latest_day: String,
    // #[serde(rename="08. previous close")]
    // prev_close: i32,
    // #[serde(rename="09. change")]
    // change: i32,
    // #[serde(rename="10. change percent")]
    // change_pct: i32
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
    let result_text = reqwest::get(&endpoint).await?.json::<GlobalQuote>().await?;
    let price = result_text.quote.price.parse::<f32>().unwrap();
    let results = format!("Last Price: {}", price);

    let _ = msg.channel_id.say(&ctx.http, results).await?;

    Ok(())
}
