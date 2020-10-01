use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("stocks", "stock", "price")]
async fn stonks(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    for stonk in args.iter::<String>() {
        let _ = msg.channel_id.say(
            &ctx.http,
            &format!(
                "https://www.finviz.com/chart.ashx?t={}&ty=c&ta=1&p=d&s=l",
                &stonk.unwrap()
            ),
        ).await;
    }
    Ok(())
}
