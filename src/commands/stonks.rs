use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("stocks", "stock", "price", "stonks", "stonk")]
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
#[aliases("sprice", "stonkprice", "stockprice")]
async fn price(ctx: &Context, msg: &Message) -> CommandResult {
    let endpoint = "https://api.adviceslip.com/advice";
    let result_text = reqwest::get(endpoint).await?.json::<Advice>().await?;
    let results = format!("{} - #{}", result_text.slip.advice, result_text.slip.id);

    let _ = msg.channel_id.say(&ctx.http, results).await;
    Ok(())
}
