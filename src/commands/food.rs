use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("cuisine", "food", "dinner", "lunch", "breakfast", "snack")]
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

    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            responses.choose(&mut rand::thread_rng()).unwrap(),
        )
        .await;

    Ok(())
}
