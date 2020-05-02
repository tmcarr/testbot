// use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;



// let mut db = PickleDb::new(
//   "testbot.db",
//   PickleDbDumpPolicy::AutoDump,
//   SerializationMethod::Json,
// );


// Command to write to DB
#[command]
fn describe(ctx: &mut Context, msg: &Message) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, "Unimplemented!");
  Ok(())
}

// Command to read from DB
#[command]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, "Unimplemented!");

  Ok(())
}

