mod commands;
mod models;
mod schema;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

use diesel::r2d2::ManageConnection;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    client::{Client, Context, EventHandler},
    framework::standard::macros::{group, help, hook},
    framework::standard::{
        help_commands, Args, CommandError, CommandGroup, CommandResult, DispatchError, HelpOptions,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
        interactions::{application_command::ApplicationCommandInteractionData, Interaction},
        prelude::UserId,
    },
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
use tracing::{debug, error, info, instrument};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::owner::*;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct AlphaVantageApiToken;
impl TypeMapKey for AlphaVantageApiToken {
    type Value = String;
}

struct PostgresClient;
impl TypeMapKey for PostgresClient {
    type Value = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
}

#[derive(Clone)]
struct SlashCommandOption {
    name: String,
    required: bool,
    description: String,
}

#[async_trait]
trait SlashCommandHandler {
    async fn handle(
        &self,
        ctx: &Context,
        data: &ApplicationCommandInteractionData,
    ) -> CommandResult<String>;
}

struct SlashCommand {
    description: &'static str,
    handler: &'static (dyn SlashCommandHandler + Send + Sync),
    options: Vec<SlashCommandOption>,
}

struct Handler {
    slash_commands: HashMap<&'static str, &'static SlashCommand>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        for guild in ready.guilds {
            for (&name, command) in self.slash_commands.iter() {
                let result = guild
                    .id()
                    .create_application_command(&ctx.http, |cmd| {
                        cmd.name(name.to_owned());
                        cmd.description(command.description.to_owned());

                        for option in &command.options {
                            cmd.create_option(  |o| {
                                o.kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::String);
                                o.name(option.name.clone());
                                o.description(option.description.clone());
                                o.required(option.required);

                            o
                            }

                            );
                        }

                        cmd
                    })
                    .await;

                match result {
                    Err(e) => error!("Could not register {}: {:?}", name, e),
                    Ok(res) => info!("Registered {}: {:?}", name, res),
                }
            }
        }
    }

    async fn interaction_create(
        &self,
        ctx: Context,
        interaction: serenity::model::interactions::Interaction,
    ) {
        debug!("Handling interaction: {:?}", interaction);

        let application_command = match interaction {
            Interaction::ApplicationCommand(ref ac) => ac,
            _ => return,
        };

        let command = match self
            .slash_commands
            .get(application_command.data.name.as_str())
        {
            Some(c) => c,
            None => return,
        };

        let response = command
            .handler
            .handle(&ctx, &application_command.data)
            .await;

        if let Ok(text) = response {
            let api_response = application_command.create_interaction_response(ctx.http, |r| {
                r.kind(serenity::model::interactions::InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(text))
            }).await;

            if let Err(e) = api_response {
                error!("Error sending bot response: {:?}", e);
            }
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(quit)]

struct General;

embed_migrations!("migrations");

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
                    .say(ctx, format!("Unrecognized command: '{}'", cmd_name))
                    .await;
            }

            // ignore any bad "commands" that aren't alphabetic
            _ => {}
        };

        println!(
            "A user named {:?} tried to executute an unknown command: {}",
            msg.author.name, cmd_name
        );
    }

    #[hook]
    async fn dispatch_error_hook(context: &Context, msg: &Message, error: DispatchError) {
        match error {
            DispatchError::NotEnoughArguments { min, given } => {
                let s = format!("Need {} arguments, but only got {}.", min, given);

                let _ = msg.channel_id.say(&context, &s).await;
            }
            DispatchError::TooManyArguments { max, given } => {
                let s = format!("Max arguments allowed is {}, but got {}.", max, given);

                let _ = msg.channel_id.say(&context, &s).await;
            }
            _ => println!("Unhandled dispatch error."),
        }
    }

    #[hook]
    #[instrument]
    async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
        //  Print out an error if it happened
        if let Err(why) = error {
            println!("Error in {}: {:?}", cmd_name, why);
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
    // the CWD. Primarially used for local testing.
    dotenv::dotenv().ok();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Failed to load DISCORD_TOKEN from environment.");
    let application_id = env::var("DISCORD_APPLICATION_ID")
        .expect("Failed to load DISCORD_APPLICATION_ID from environment.")
        .parse()
        .expect("Application ID must be an integer");
    let alphavantage_token =
        env::var("ALPHAVANTAGE").expect("Failed to retrieve alphavantage API token.");
    let database_url = env::var("DATABASE_URL").expect("Unable to read Database URL.");
    let http = Http::new_with_token_application_id(&token, application_id);

    // Create DB client
    let connection_manager = diesel::r2d2::ConnectionManager::new(database_url);

    let db_client = connection_manager
        .connect()
        .expect("Could not connect to Postgres");

    embedded_migrations::run(&db_client).expect("Could not run migrations");

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let slash_commands = maplit::hashmap! {
        "8ball" => &*commands::ball::BALL_COMMAND,
        "advice" => &commands::advice::ADVICE_COMMAND,
        "botsnack" => &commands::botsnack::BOTSNACK_COMMAND,
        "company" => &*commands::stonks::COMPANY_COMMAND,
        "cuisine" => &commands::food::FOOD_COMMAND,
        "define" => &*commands::desc::DEFINE_COMMAND,
        "describe" => &*commands::desc::DESCRIBE_COMMAND,
        "drink" => &commands::drink::DRINK_COMMAND,
        "fart" => &commands::pingpong::FART_COMMAND,
        "ping" => &commands::pingpong::PING_COMMAND,
        "price" => &*commands::stonks::PRICE_COMMAND,
        "random" => &*commands::random::RANDOM_COMMAND,
        "source" => &commands::github::GITHUB_COMMAND,
        "stonks" => &*commands::stonks::STONKS_COMMAND,
        "stonkcomp" => &*commands::stonks::STONKCOMP_COMMAND,
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .prefix("~")
                .on_mention(Some(_bot_id))
                .ignore_webhooks(true)
                .ignore_bots(true)
                .case_insensitivity(true)
        })
        .before(before_hook)
        .after(after_hook)
        .unrecognised_command(unrecognized_command_hook)
        .on_dispatch_error(dispatch_error_hook)
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let mut client = Client::builder(&token)
        .application_id(application_id)
        .framework(framework)
        .event_handler(Handler { slash_commands })
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<AlphaVantageApiToken>(alphavantage_token);
        data.insert::<PostgresClient>(connection_manager);
    };
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    };
}
