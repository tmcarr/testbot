use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("drink", "drinks", "drank")]
async fn drink(ctx: &Context, msg: &Message) -> CommandResult {
    let responses = vec![
        "Water.",
        "Topo Chico",
        "La Croix",
        "Water?",
        "Milk",
        "Sparking Water",
        "Seltzer Water",
        "Tap Water",
        "Voda",
        "Dihydrogen monoxide",
        "Vand",
        "Eau",
        "Akvo",
        "Agua",
    ];

    let drink = responses.choose(&mut rand::thread_rng()).unwrap();

    let _ = msg.channel_id.say(&ctx.http, drink).await;

    Ok(())
}
