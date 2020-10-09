use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("drink", "drinks")]
fn drink(ctx: &mut Context, msg: &Message) -> CommandResult {
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

    if let Err(e) = msg.channel_id.say(
        &ctx.http,
        responses.choose(&mut rand::thread_rng()).unwrap(),
    ) {
        return Err(CommandError::from(e));
    }
    Ok(())
}
