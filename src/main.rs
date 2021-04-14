mod commands;

// use dhb_postgres_heroku::{get_pool, HerokuPool};
use log::error;
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
    model::{channel::Message, event::ResumedEvent, gateway::Ready, prelude::UserId},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};
use tracing::{info, instrument};

// Re import desc::*,  when its ready
use commands::{
    advice::*, ball::*, botsnack::*, drink::*, food::*, github::*, meta::*, owner::*, random::*,
    stonks::*,
};

use dhb_postgres_heroku::{get_client, Client as HerokuPostgresClient};

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct AlphaVantageAPIToken;
impl TypeMapKey for AlphaVantageAPIToken {
    type Value = String;
}

struct HerokuPostgresConnectionPool;
impl TypeMapKey for HerokuPostgresConnectionPool {
    type Value = HerokuPostgresClient;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

// Remember to re-add advice here when its ready.
#[group]
#[commands(
    advice,
    ball,
    botsnack,
    description,
    drink,
    food,
    github,
    ping,
    price,
    quit,
    random,
    stonkcomp,
    stonks
)]
struct General;

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
    kankyo::init().ok();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Failed to load DISCORD_TOKEN from environment.");
    let alphavantage_token =
        env::var("ALPHAVANTAGE").expect("Failed to retrieve alphavantage API token.");
    let database_url = env::var("DATABASE_URL").expect("Unable to read Database URL.");
    let http = Http::new_with_token(&token);

    // Create DB client
    let db_client = get_client(&database_url);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
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
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<AlphaVantageAPIToken>(alphavantage_token);
        data.insert::<HerokuPostgresConnectionPool>(db_client);
    };
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    };
}
