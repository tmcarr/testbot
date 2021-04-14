use crate::PostgresClient;
use diesel::r2d2::ManageConnection;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::schema::descriptions::dsl::*;
use crate::models::Description;
use crate::diesel::RunQueryDsl;

#[command]
#[aliases("set")]
async fn describe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let input_key = args.single::<String>().unwrap();
    let input_value = args.single::<String>().unwrap();

    if let Some(dbclient) = data.get::<PostgresClient>() {
        let db_connection = dbclient.connect().expect("Could not connect to Postgres");
        let _ = msg
            .channel_id
            .say(&ctx.http, &format!("Defining {} as: '{}'", &input_key, &input_value))
            .await;

        let description = Description {
          key: &input_key,
          value: &input_value,
        };
        // Do DB Write here
        diesel::insert_into(descriptions)
            .values(&description)
            .execute(&db_connection)
            .unwrap();
    } else {
        msg.reply(
            ctx,
            &format!(
                "There was a problem reading from the databse. Failed to define {} as '{}'",
                &input_key, &input_value
            ),
        )
        .await?;

        return Ok(());
    };
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
