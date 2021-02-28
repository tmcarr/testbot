use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("source")]
#[description = "Reply with a link to the bot's source code"]
#[usage = ""]
async fn github(ctx: &Context, msg: &Message) -> CommandResult {
    let github = "https://github.com/tmcarr/testbot";
    let _ = msg
        .channel_id
        .say(&ctx.http, &format!("My code is at: {}", &github))
        .await;
    Ok(())
}
