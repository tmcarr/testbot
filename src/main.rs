mod commands;
mod models;
mod schema;

// #[macro_use]
// extern crate diesel_migrations;

// #[macro_use]
// extern crate diesel;
// use diesel::pg::Pg;
// use diesel::r2d2::ManageConnection;
use dotenvy::dotenv;
use std::env;
use tracing::instrument;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::{
    advice::*, ball::*, botsnack::*, dogfacts::*, drink::*, food::*, github::*, owner::*,
    pingpong::*, random::*, stonks::*,
};

// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// Data shared between command invocations
pub struct Data {
    pub alphavantage_token: String,
}

// Error type for the bot
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Register slash commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
This is an example bot made to showcase features for my friends",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

#[tokio::main]
#[instrument]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize the logger
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    // Get environment variables
    let token = env::var("DISCORD_TOKEN").expect("Failed to load DISCORD_TOKEN from environment.");
    let alphavantage_token =
        env::var("ALPHAVANTAGE").expect("Failed to retrieve alphavantage API token.");

    let options = poise::FrameworkOptions {
        commands: vec![
            // Built-in commands
            register(),
            help(),
            // Custom commands
            advice(),
            ball(),
            botsnack(),
            dogfact(),
            drink(),
            fart(),
            food(),
            github(),
            ping(),
            quit(),
            random(),
            stonks(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            ..Default::default()
        },
        // This code runs before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().name);
            })
        },
        // This code runs after every command
        post_command: |ctx| {
            Box::pin(async move {
                println!("Finished command {}!", ctx.command().name);
            })
        },
        // This code runs when a command produces an error
        on_error: |error| {
            Box::pin(async move {
                match error {
                    poise::FrameworkError::Setup { error, .. } => {
                        panic!("Failed to start bot: {:?}", error)
                    }
                    poise::FrameworkError::Command { error, ctx, .. } => {
                        println!("Error in command `{}`: {:?}", ctx.command().name, error,);
                    }
                    error => {
                        if let Err(e) = poise::builtins::on_error(error).await {
                            println!("Error while handling error: {}", e)
                        }
                    }
                }
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { alphavantage_token })
            })
        })
        .build();

    let intents = poise::serenity_prelude::GatewayIntents::non_privileged()
        | poise::serenity_prelude::GatewayIntents::MESSAGE_CONTENT;
    let mut client = poise::serenity_prelude::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");
    client.start().await.expect("Client error");
}
