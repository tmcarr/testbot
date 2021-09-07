use crate::SlashCommand;

use serde::Deserialize;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

#[derive(Deserialize)]
struct Slip {
    id: i32,
    advice: String,
}

#[derive(Deserialize)]
struct Advice {
    slip: Slip,
}

async fn advice(_: &Context, _: &ApplicationCommandInteractionData) -> CommandResult<String> {
    const ENDPOINT: &str = "https://api.adviceslip.com/advice";
    let advice = reqwest::get(ENDPOINT).await?.json::<Advice>().await?;
    let results = format!("{} - #{}", advice.slip.advice, advice.slip.id);
    Ok(results)
}

make_slash_command_handler!(AdviceHandler, advice);

pub(crate) static ADVICE_COMMAND: SlashCommand = SlashCommand {
    description: "Asks for the advice of the gods and reveals their musings.",
    handler: &AdviceHandler,
    options: vec![],
};
