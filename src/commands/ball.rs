use crate::{SlashCommand, SlashCommandOption};

use rand::seq::SliceRandom;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

async fn ball(_: &Context, _: &ApplicationCommandInteractionData) -> CommandResult<String> {
    let responses = vec![
        "As I see it, yes.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don’t count on it.",
        "It is certain.",
        "It is decidedly so.",
        "Most likely.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Outlook good.",
        "Reply hazy, try again.",
        "Signs point to yes.",
        "Very doubtful.",
        "Without a doubt.",
        "Yes.",
        "Yes – definitely.",
        "You may rely on it.",
    ];

    let choice = responses.choose(&mut rand::thread_rng()).unwrap();

    Ok(choice.to_string())
}

make_slash_command_handler!(BallHandler, ball);

lazy_static::lazy_static! {
    pub(crate) static ref BALL_COMMAND: SlashCommand = SlashCommand {
        description: "Shakes the digital 8-ball.",
        handler: &BallHandler,
        options: vec![
            SlashCommandOption {
                name: "question".to_string(),
                required: true,
                description: "What would you like to ask the magic 8-ball?".to_string(),
            },
        ]
    };
}
