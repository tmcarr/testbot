use rand::seq::IteratorRandom;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "A bot's gotta eat...."]
#[usage = ""]
async fn botsnack(ctx: &Context, msg: &Message) -> CommandResult {
    let responses = ["Yum!", "*cronch*", "MOAR", "*Smiles*", "Nice."];
    let response = responses.iter().choose(&mut rand::thread_rng()).unwrap();

    let _ = msg.channel_id.say(&ctx.http, response).await;

    Ok(())
}
