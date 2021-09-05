use crate::SlashCommand;

use rand::seq::SliceRandom;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

async fn botsnack(_: &Context, _: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let responses = vec!["Yum!", "*cronch*", "MOAR", "*Smiles*", "Nice."];
    let response = *responses.choose(&mut rand::thread_rng()).unwrap();

    Ok(response.to_owned())
}

make_slash_command_handler!(BotsnackHandler, botsnack);

pub(crate) static BOTSNACK_COMMAND: SlashCommand = SlashCommand {
    description: "A bot's gotta eat....",
    handler: &BotsnackHandler,
    options: vec![],
};
