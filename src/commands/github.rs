use crate::SlashCommand;

use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

async fn github(_: &Context, _: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let github = "https://github.com/tmcarr/testbot";
    Ok(format!("My code is at: {}", &github))
}

make_slash_command_handler!(GithubHandler, github);

pub(crate) static GITHUB_COMMAND: SlashCommand = SlashCommand {
    description: "Reply with a link to the bot's source code",
    handler: &GithubHandler,
    options: vec![],
};
