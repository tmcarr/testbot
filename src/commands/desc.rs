use crate::PostgresClient;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("set")]
async fn describe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let key = args.single::<String>().unwrap();
    let value = args.single::<String>().unwrap();

    if let Some(_dbclient) = data.get::<PostgresClient>() {
        let _ = msg
            .channel_id
            .say(&ctx.http, &format!("Defining {} as: '{}'", &key, &value))
            .await;
    } else {
        msg.reply(
            ctx,
            &format!(
                "There was a problem reading from the databse. Failed to define {} as '{}'",
                &key, &value
            ),
        )
        .await?;

        return Ok(());
    }
    Ok(())
}

// #[command]
// #[aliases("whatis", "show")]
// async fn define(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
//     let data = ctx.data.read().await;
//     let key = args.single::<String>().unwrap();

//     if let Some(dbclient) = data.get::<PostgresClient>() {
//         // Read from DB
//         let _ = msg
//             .channel_id
//             .say(
//                 &ctx.http,
//                 &format!("INCOMPELTE. LIES: {} is decribed as: '{}'", key, value),
//             )
//             .await;
//     } else {
//         msg.reply(
//             ctx,
//             &format!(
//                 "There was a problem getting the databse client. Failed to define {} as '{}'",
//                 &key, &value
//             ),
//         )
//         .await?;

//         return Ok(());
//     }
//     Ok(())
// }
