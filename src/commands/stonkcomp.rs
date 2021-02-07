use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

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
