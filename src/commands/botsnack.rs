use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
fn botsnack(ctx: &mut Context, msg: &Message) -> CommandResult {
    let responses = vec!["Yum!", "*cronch*", "MOAR", "*Smiles*"];

    let _ = msg.channel_id.say(
        &ctx.http,
        responses.choose(&mut rand::thread_rng()).unwrap(),
    );

    Ok(())
}
