use crate::SlashCommand;

use rand::seq::SliceRandom;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

async fn drink(_: &Context, _: &ApplicationCommandInteractionData) -> CommandResult<String> {
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

    Ok(drink.to_string())
}

make_slash_command_handler!(DrinkHandler, drink);

pub(crate) static DRINK_COMMAND: SlashCommand = SlashCommand {
    description: "Reply with a suggestion of fine beverage.",
    handler: &DrinkHandler,
    options: vec![],
};
