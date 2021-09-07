use itertools::Itertools;
use serde::de;
use serde::{self, Deserialize, Deserializer};
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fmt::Display;
use std::str::FromStr;

use crate::AlphaVantageApiToken;
use crate::{SlashCommand, SlashCommandOption};

lazy_static::lazy_static! {
    static ref SINGLE_STOCK: Vec<SlashCommandOption> = vec![SlashCommandOption {
        name: "stock".to_string(),
        description: "ticker symbol".to_string(),
        required: true,
    }];

    static ref MULTIPLE_STOCKS: Vec<SlashCommandOption> = {
        let mut options = SINGLE_STOCK.clone();

        options.extend((2..=25).map(|i| SlashCommandOption {
            name: format!("stock{}", i),
            description: "ticker symbol".to_string(),
            required: false,
        }));

        options
    };

    // This lets us iterate through the arguments in the same order the user would have specified
    // them, as otherwise the HashMap iterates in an a random order.
    static ref MULTIPLE_STOCKS_KEYS: Vec<String> = {
        let mut keys = vec!["stock".to_owned()];
        keys.extend((2..=25).map(|i| format!("stock{}", i)));
        keys
    };
}

// TODO: #[aliases("stocks", "stock", "stonks", "stonk")]
async fn stonks(_ctx: &Context, data: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let arguments = super::get_string_arguments(data);
    let mut reply = vec![];
    for key in MULTIPLE_STOCKS_KEYS.iter() {
        if let Some(&stonk) = arguments.get(key.as_str()) {
            reply.push(format!(
                "https://www.finviz.com/chart.ashx?t={}&ty=c&ta=1&p=d&s=l",
                stonk
            ))
        }
    }
    Ok(reply.iter().join("\n"))
}

make_slash_command_handler!(StonksHandler, stonks);

lazy_static::lazy_static! {
    pub(crate) static ref STONKS_COMMAND: SlashCommand = SlashCommand {
        description: "Display the Finviz graph for a given ticker.",
        options: MULTIPLE_STOCKS.clone(),
        handler: &StonksHandler,
    };
}

// TODO: #[aliases("stockcomp", "s&pcomp")]
async fn stonkcomp(
    _ctx: &Context,
    data: &ApplicationCommandInteractionData,
) -> CommandResult<String> {
    let arguments = super::get_string_arguments(data);
    let mut reply = vec![];
    for key in MULTIPLE_STOCKS_KEYS.iter() {
        if let Some(&stonk) = arguments.get(key.as_str()) {
            reply.push(format!("https://stonks.egd.pw/spcomp?symbol={}", stonk));
        }
    }
    Ok(reply.iter().join("\n"))
}

make_slash_command_handler!(StonkcompHandler, stonkcomp);

lazy_static::lazy_static! {
    pub(crate) static ref STONKCOMP_COMMAND: SlashCommand = SlashCommand {
        description: "Display a graphic showing performance information about a ticker compared to the S&P500",
        options: MULTIPLE_STOCKS.clone(),
        handler: &StonkcompHandler,
    };
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

// TODO: #[aliases("p")]
async fn price(ctx: &Context, data: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let api_token = get_api_token(ctx).await;
    let arguments = super::get_string_arguments(data);
    let ticker = arguments
        .get("stock")
        .ok_or_else(|| Box::new(super::CommandError::OptionMissing))?;
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

    Ok(message)
}

make_slash_command_handler!(PriceHandler, price);

lazy_static::lazy_static! {
    pub(crate) static ref PRICE_COMMAND: SlashCommand = SlashCommand {
        description: "Find price information about a ticker",
        options: SINGLE_STOCK.clone(),
        handler: &PriceHandler,
    };
}

// Example output here: https://www.alphavantage.co/query?function=OVERVIEW&symbol=IBM&apikey=demo
#[derive(Deserialize, Debug)]
struct Overview {
    #[serde(rename(deserialize = "Symbol"))]
    symbol: String,
    #[serde(rename(deserialize = "AssetType"))]
    assettype: String,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Description"))]
    description: String,
    #[serde(rename(deserialize = "Exchange"))]
    exchange: String,
    #[serde(rename(deserialize = "Currency"))]
    currency: String,
    #[serde(rename(deserialize = "Country"))]
    country: String,
    #[serde(rename(deserialize = "Sector"))]
    sector: String,
    #[serde(rename(deserialize = "Industry"))]
    industry: String,
    #[serde(rename(deserialize = "Address"))]
    address: String,
    #[serde(rename(deserialize = "FullTimeEmployees"))]
    fulltimeemployees: String,
    #[serde(rename(deserialize = "FiscalYearEnd"))]
    fiscalyearend: String,
    #[serde(rename(deserialize = "LatestQuarter"))]
    latestquarter: String,
    #[serde(rename(deserialize = "MarketCapitalization"))]
    marketcapitalization: String,
    #[serde(rename(deserialize = "EBITDA"))]
    ebitda: String,
    #[serde(rename(deserialize = "PERatio"))]
    peratio: String,
    #[serde(rename(deserialize = "PEGRatio"))]
    pegratio: String,
    #[serde(rename(deserialize = "BookValue"))]
    bookvalue: String,
    #[serde(rename(deserialize = "DividendPerShare"))]
    dividendpershare: String,
    #[serde(rename(deserialize = "DividendYield"))]
    dividendyield: String,
    #[serde(rename(deserialize = "EPS"))]
    eps: String,
    #[serde(rename(deserialize = "RevenuePerShareTTM"))]
    revenuepersharettm: String,
    #[serde(rename(deserialize = "ProfitMargin"))]
    profitmargin: String,
    #[serde(rename(deserialize = "OperatingMarginTTM"))]
    operatingmarginttm: String,
    #[serde(rename(deserialize = "ReturnOnAssetsTTM"))]
    returnonassetsttm: String,
    #[serde(rename(deserialize = "ReturnOnEquityTTM"))]
    returnonequityttm: String,
    #[serde(rename(deserialize = "RevenueTTM"))]
    revenuettm: String,
    #[serde(rename(deserialize = "GrossProfitTTM"))]
    grossprofitttm: String,
    #[serde(rename(deserialize = "DilutedEPSTTM"))]
    dilutedepsttm: String,
    #[serde(rename(deserialize = "QuarterlyEarningsGrowthYOY"))]
    quarterlyearningsgrowthyoy: String,
    #[serde(rename(deserialize = "QuarterlyRevenueGrowthYOY"))]
    quarterlyrevenuegrowthyoy: String,
    #[serde(rename(deserialize = "AnalystTargetPrice"))]
    analysttargetprice: String,
    #[serde(rename(deserialize = "TrailingPE"))]
    trailingpe: String,
    #[serde(rename(deserialize = "ForwardPE"))]
    forwardpe: String,
    #[serde(rename(deserialize = "PriceToSalesRatioTTM"))]
    pricetosalesratiottm: String,
    #[serde(rename(deserialize = "PriceToBookRatio"))]
    pricetobookratio: String,
    #[serde(rename(deserialize = "EVToRevenue"))]
    evtorevenue: String,
    #[serde(rename(deserialize = "EVToEBITDA"))]
    evtoebitda: String,
    #[serde(rename(deserialize = "Beta"))]
    beta: String,
    #[serde(rename(deserialize = "52WeekHigh"))]
    fiftytwoweekhigh: String,
    #[serde(rename(deserialize = "52WeekLow"))]
    fiftytwoweeklow: String,
    #[serde(rename(deserialize = "50DayMovingAverage"))]
    fiftydaymovingaverage: String,
    #[serde(rename(deserialize = "200DayMovingAverage"))]
    twohundreddaymovingaverage: String,
    #[serde(rename(deserialize = "SharesOutstanding"))]
    sharesoutstanding: String,
    #[serde(rename(deserialize = "SharesFloat"))]
    sharesfloat: String,
    #[serde(rename(deserialize = "SharesShort"))]
    sharesshort: String,
    #[serde(rename(deserialize = "SharesShortPriorMonth"))]
    sharesshortpriormonth: String,
    #[serde(rename(deserialize = "ShortRatio"))]
    shortratio: String,
    #[serde(rename(deserialize = "ShortPercentOutstanding"))]
    shortpercentoutstanding: String,
    #[serde(rename(deserialize = "ShortPercentFloat"))]
    shortpercentfloat: String,
    #[serde(rename(deserialize = "PercentInsiders"))]
    percentinsiders: String,
    #[serde(rename(deserialize = "PercentInstitutions"))]
    percentinstitutions: String,
    #[serde(rename(deserialize = "ForwardAnnualDividendRate"))]
    forwardannualdividendrate: String,
    #[serde(rename(deserialize = "ForwardAnnualDividendYield"))]
    forwardannualdividendyield: String,
    #[serde(rename(deserialize = "PayoutRatio"))]
    payoutratio: String,
    #[serde(rename(deserialize = "DividendDate"))]
    dividenddate: String,
    #[serde(rename(deserialize = "ExDividendDate"))]
    exdividenddate: String,
    #[serde(rename(deserialize = "LastSplitFactor"))]
    lastsplitfactor: String,
    #[serde(rename(deserialize = "LastSplitDate"))]
    lastsplitdate: String,
}

async fn company(ctx: &Context, data: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let api_token = get_api_token(ctx).await;
    let arguments = super::get_string_arguments(data);
    let ticker = arguments
        .get("stock")
        .ok_or_else(|| Box::new(super::CommandError::OptionMissing))?;
    let endpoint = format!(
        "https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",
        ticker, api_token
    );
    let profile = reqwest::get(&endpoint).await?.json::<Overview>().await?;

    Ok(profile.description)
}

make_slash_command_handler!(CompanyHandler, company);

lazy_static::lazy_static! {
    pub(crate) static ref COMPANY_COMMAND: SlashCommand = SlashCommand {
        description:  "Find a summary of a company from its ticker.",
        options: SINGLE_STOCK.clone(),
        handler: &PriceHandler,
    };
}

async fn get_api_token(ctx: &Context) -> String {
    let data_read = ctx.data.read().await;
    let api_token = data_read
        .get::<AlphaVantageApiToken>()
        .expect("Expected an AlphaVantage API token in the context.");

    api_token.clone()
}
