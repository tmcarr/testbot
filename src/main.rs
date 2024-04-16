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
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::macros::{group, help, hook},
    framework::standard::{
        help_commands, Args, CommandError, CommandGroup, CommandResult, DispatchError, HelpOptions,
        StandardFramework,
    },
    http::Http,
    model::{
        application::command::CommandOptionType,
        application::interaction::application_command::CommandDataOptionValue,
        application::interaction::Interaction,
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
        id::GuildId,
        prelude::{interaction::InteractionResponseType, UserId},
    },
    prelude::*,
};
// use std::error::Error;
use std::{collections::HashSet, env, sync::Arc};
use tracing::{error, info, instrument};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::{
    advice::*, ball::*, botsnack::*, drink::*, food::*, github::*, owner::*, pingpong::*,
    random::*, stonks::*,
};

// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct AlphaVantageApiToken;
impl TypeMapKey for AlphaVantageApiToken {
    type Value = String;
}

// struct PostgresClient;
// impl TypeMapKey for PostgresClient {
//     type Value = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
// }

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let guild_id = GuildId(ready.guilds[0].id.0);

        let _slashcommands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("id")
                        .description("Get a user id")
                        .create_option(|option| {
                            option
                                .name("id")
                                .description("The user to lookup")
                                .kind(CommandOptionType::User)
                                .required(true)
                        })
                })
        });
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let CommandDataOptionValue::User(user, _member) = options {
                        format!("{}'s id is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
                    }
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }
}

#[group]
#[commands(
    advice,
    ball,
    botsnack,
    // define,
    // describe,
    // description,
    drink,
    fart,
    food,
    github,
    ping,
    // price,
    quit,
    random,
    stonkcomp,
    stonks
)]

struct General;

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
#[instrument]
async fn main() {
    #[hook]
    #[instrument]
    async fn before_hook(_: &Context, msg: &Message, cmd_name: &str) -> bool {
        println!(
            "Got command '{}' by user '{}': '{}'",
            cmd_name, msg.author, msg.content
        );
        true
    }

    #[hook]
    async fn unrecognized_command_hook(ctx: &Context, msg: &Message, cmd_name: &str) {
        match cmd_name.chars().next() {
            Some(x) if x.is_alphabetic() => {
                let _ = msg
                    .channel_id
                    .say(ctx, format!("Unrecognized command: '{cmd_name}'"))
                    .await;
            }

            // ignore any bad "commands" that aren't alphabetic
            _ => {}
        };

        println!(
            "A user named {:?} tried to execute an unknown command: {}",
            msg.author.name, cmd_name
        );
    }

    #[hook]
    async fn dispatch_error(
        ctx: &Context,
        msg: &Message,
        error: DispatchError,
        _command_name: &str,
    ) {
        if let DispatchError::Ratelimited(info) = error {
            // We notify them only once.
            if info.is_first_try {
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", info.as_secs()),
                    )
                    .await;
            }
        }
    }

    #[hook]
    #[instrument]
    async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
        //  Print out an error if it happened
        if let Err(why) = error {
            println!("Error in {cmd_name}: {why:?}");
        }
    }

    #[help]
    async fn my_help(
        context: &Context,
        msg: &Message,
        args: Args,
        help_options: &'static HelpOptions,
        groups: &[&'static CommandGroup],
        owners: HashSet<UserId>,
    ) -> CommandResult {
        let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
        Ok(())
    }

    // This will load the environment variables located at `./.env`, relative to
    // the CWD. Primarily used for local testing.
    // dotenv::dotenv().ok();
    dotenv().ok();
    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let app_id = env::var("DISCORD_APP_ID")
        .expect("Failed to load DISCORD_APP_ID from the environment.")
        .parse::<u64>()
        .expect("Failed to parse DISCORD_APP_ID into u64.");
    let token = env::var("DISCORD_TOKEN").expect("Failed to load DISCORD_TOKEN from environment.");
    let alphavantage_token =
        env::var("ALPHAVANTAGE").expect("Failed to retrieve alphavantage API token.");
    // let database_url = env::var("DATABASE_URL").expect("Unable to read Database URL.");
    let http = Http::new(&token);

    // Create DB client
    // let connection_manager: diesel::r2d2::ConnectionManager<diesel::pg::PgConnection> =
    //     diesel::r2d2::ConnectionManager::new(database_url);

    // let mut db_client: _ = connection_manager
    //     .connect()
    //     .expect("Could not connect to Postgres");

    // fn run_migrations(
    //     connection: &mut impl MigrationHarness<Pg>,
    // ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    //     // This will run the necessary migrations.
    //     //
    //     // See the documentation for `MigrationHarness` for
    //     // all available methods.
    //     connection.run_pending_migrations(MIGRATIONS)?;

    //     Ok(())
    // }

    // run_migrations(&mut db_client).ok();

    // Fetch bots owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {why:?}"),
            }
        }
        Err(why) => panic!("Could not access application info: {why:?}"),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .prefix("~")
                .allow_dm(true)
                .on_mention(Some(bot_id))
                .ignore_webhooks(true)
                .ignore_bots(true)
                .no_dm_prefix(true)
                .case_insensitivity(true)
        })
        .before(before_hook)
        .after(after_hook)
        .unrecognised_command(unrecognized_command_hook)
        .on_dispatch_error(dispatch_error)
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .application_id(app_id)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<AlphaVantageApiToken>(alphavantage_token);
        // data.insert::<PostgresClient>(connection_manager);
    };
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    };
}
