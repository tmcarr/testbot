use crate::PostgresClient;
use crate::ShardManagerContainer;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[owners_only]
#[aliases("killkillkill", "abortabortabort")]
#[description = "Causes the bot to die."]
#[usage = ""]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down!").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager")
            .await?;

        return Ok(());
    }

    Ok(())
}

#[command]
#[owners_only]
#[description = "Initializes DB Tables"]
#[usage = ""]
async fn initdb(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(_dbclient) = data.get::<PostgresClient>() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "Initialized database tables.".to_string())
            .await;
    } else {
        msg.reply(ctx, "Unable to initialize the databse tables.".to_string())
            .await?;

        return Ok(());
    }

    Ok(())
}
