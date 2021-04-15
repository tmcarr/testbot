use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("cuisine", "dinner", "lunch", "breakfast", "snack")]
#[description = "Reply with a suggestion for cuisine."]
#[usage = ""]
async fn food(ctx: &Context, msg: &Message) -> CommandResult {
    let responses = vec![
        "Asian",
        "Barbecue",
        "Hamburgers",
        "Italian",
        "Mexican",
        "Pho",
        "Pizza",
        "Steak",
        "Seafood",
        "Indian",
        "Cajun",
    ];

    let item = responses.choose(&mut rand::thread_rng()).unwrap();

    let _ = msg.channel_id.say(&ctx.http, item).await;

    Ok(())
}
