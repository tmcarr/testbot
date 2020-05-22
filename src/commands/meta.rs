use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}
