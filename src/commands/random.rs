use crate::{SlashCommand, SlashCommandOption};

use rand::seq::IteratorRandom;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

// TODO: #[aliases("rand")]
async fn random(_ctx: &Context, data: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let arguments = super::get_string_arguments(data);

    let thing = arguments.values().choose(&mut rand::thread_rng());

    let response = match thing {
        Some(&choice) => choice.to_owned(),
        _ => "Why u no args?!".to_owned(),
    };

    Ok(response)
}

make_slash_command_handler!(RandomHandler, random);

lazy_static::lazy_static! {
    pub(crate) static ref RANDOM_COMMAND: SlashCommand = SlashCommand {
        description: "Choose a random item from the list of inputs",
        options: (1..=25)
            .map(|i| SlashCommandOption {
                name: format!("choice{}", i),
                required: false,
                description: format!("Choice {}", i),
            })
            .collect(),
        handler: &RandomHandler,
    };
}
