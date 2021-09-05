use crate::{async_trait, SlashCommand, SlashCommandHandler};

use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

struct StaticReplyHandler(&'static str);

#[async_trait]
impl SlashCommandHandler for StaticReplyHandler {
    async fn handle(
        &self,
        _: &Context,
        _: &ApplicationCommandInteractionData,
    ) -> CommandResult<String> {
        Ok(self.0.to_string())
    }
}

pub(crate) static FART_COMMAND: SlashCommand = SlashCommand {
    description: "Let er' rip!",
    handler: &StaticReplyHandler("Thbbbbbbbbbbbbbbt.... squeak."),
    options: vec![],
};

pub(crate) static PING_COMMAND: SlashCommand = SlashCommand {
    description: "Reploy with 'Pong!'",
    handler: &StaticReplyHandler("Pong!"),
    options: vec![],
};
