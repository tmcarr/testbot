use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Reploy with 'Pong!'"]
#[usage = ""]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!").await;

    Ok(())
}

#[command]
#[description = "Let er' rip!"]
#[usage = ""]
async fn fart(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(&ctx.http, "Thbbbbbbbbbbbbbbt.... squeak.")
        .await;

    Ok(())
}
