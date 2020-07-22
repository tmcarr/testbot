//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
mod commands;

// use dhb_postgres_heroku::{get_pool, HerokuPool};
use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{macros::group, DispatchError, StandardFramework},
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use commands::{advice::*, ball::*, desc::*, github::*, math::*, meta::*, owner::*};

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// struct DbClient;
// impl TypeMapKey for DbClient {
//     type Value = HerokuPool;
// }

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(advice, ball, describe, about, add, multiply, ping, quit, github)]
struct General;

fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load().ok();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Failed to load DISCORD_TOKEN from environment.");

    // Database pool setup
    // let database_url =
    //     env::var("DATABASE_URL").expect("Failed to load DATABSE_URL from environment.");
    // let max_pool_size = 20;
    // let db_pool = get_pool(&database_url, max_pool_size);

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        // data.insert::<DbClient>(db_pool);
    }

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.on_mention(Some(bot_id)).owners(owners).prefix("~"))
            .before(|_ctx, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}': '{}'",
                    command_name, msg.author, msg.content,
                );
                true // if `before` returns false, command processing doesn't happen.
            })
            .unrecognised_command(|ctx, msg, unknown_command_name| {
                let _ = msg.channel_id.say(
                    &ctx.http,
                    &format!("Could not find command: {}", unknown_command_name),
                );
                println!("Could not find command named '{}'", unknown_command_name);
            })
            .on_dispatch_error(|ctx, msg, error| {
                if let DispatchError::Ratelimited(seconds) = error {
                    let _ = msg.channel_id.say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", seconds),
                    );
                }
                if let DispatchError::OnlyForOwners = error {
                    let _ = msg
                        .channel_id
                        .say(&ctx.http, "You dont have permission do to that.");
                }
            })
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
