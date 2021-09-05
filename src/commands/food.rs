use crate::SlashCommand;

use rand::seq::SliceRandom;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

// TODO: #[aliases("cuisine", "dinner", "lunch", "breakfast", "snack")]
async fn food(_: &Context, _: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let responses = vec![
        "Asian", "Barbecue", "Burgers", "Italian", "Mexican", "Pho", "Pizza", "Steak", "Seafood",
        "Indian", "Cajun",
    ];

    let item = responses.choose(&mut rand::thread_rng()).unwrap();

    Ok(item.to_string())
}

make_slash_command_handler!(FoodHandler, food);

pub(crate) static FOOD_COMMAND: SlashCommand = SlashCommand {
    description: "Reply with a suggestion for cuisine.",
    handler: &FoodHandler,
    options: vec![],
};
