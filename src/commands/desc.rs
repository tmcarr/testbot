// use crate::DbClient;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

// Command to write to DB
#[command]
fn describe(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    // let _pool = data
    //     .get::<DbClient>()
    //     .expect("Failed to get database pool from context");

    let value = &args.message();

    let mut db = PickleDb::load(
        "testbot.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    )
    .unwrap();

    db.set(&String::from(&msg.author.name), value).unwrap();
    let _ = msg.channel_id.say(
        &ctx.http,
        &format!("Set {}'s description to: '{}'", &msg.author.name, value),
    );
    Ok(())
}

// Command to read from DB
#[command]
fn about(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = &args.single_quoted::<String>().unwrap();

    let db = PickleDb::load(
        "testbot.db",
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    )
    .unwrap();

    let description = db.get::<String>(user).unwrap();

    let _ = msg.channel_id.say(
        &ctx.http,
        &format!("{} is decribed as: '{}'", user, description),
    );

    Ok(())
}
