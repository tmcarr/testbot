use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serenity::framework::standard::{
  Args, macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

// Command to write to DB
#[command]
fn describe(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let mut db = PickleDb::new(
    "testbot.db",
    PickleDbDumpPolicy::AutoDump,
    SerializationMethod::Json,
  );

  let value = &args.parse::<String>().unwrap();
  db.set(&String::from(&msg.author.name), value).unwrap();
  let _ = msg.channel_id.say(&ctx.http, &format!("Set {}'s description to: '{}'", &msg.author.name, value));
  Ok(())
}

// Command to read from DB
#[command]
fn about(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, "Unimplemented!");

  Ok(())
}

