use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn describe(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // let data = ctx.data.read();

    let value = &args.message();

    if let Some(client) = data.get::<HerokuPostgresClient>() {
        msg.reply(ctx, "Shutting down!").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager")
            .await?;

        return Ok(());
    }


    // Set in DB
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            &format!("Set {}'s description to: '{}'", &msg.author.name, value),
        )
        .await;
    Ok(())
}

#[command]
async fn define(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = &args.single_quoted::<String>().unwrap();

    // Read from DB
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            &format!("{} is decribed as: '{}'", user, user),
        )
        .await;

    Ok(())
}
