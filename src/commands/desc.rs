use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::*;
use crate::schema::descriptions::dsl::*;
use crate::PostgresClient;
use diesel::r2d2::ManageConnection;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("set")]
async fn describe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let input_key = args.single::<String>().unwrap();
    let input_value = args.remains().unwrap();

    if let Some(dbclient) = data.get::<PostgresClient>() {
        let connection = dbclient.connect().expect("Could not connect to Postgres");
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                &format!("Defining {} as: '{}'", &input_key, &input_value),
            )
            .await;

        let description = NewDescription {
            key: &input_key,
            value: &input_value,
        };
        diesel::insert_into(descriptions)
            .values(&description)
            .on_conflict(key)
            .do_update()
            .set(&description)
            .execute(&connection)?;
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

#[command]
#[aliases("show")]
async fn define(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let input_key = args.single::<String>().unwrap();

    if let Some(dbclient) = data.get::<PostgresClient>() {
        let connection = dbclient.connect().expect("Could not connect to Postgres");

        // Do DB Read here
        let value_data = descriptions
            .filter(key.eq(&input_key))
            .load::<Description>(&connection)
            .expect("Error loading results.");

        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                &format!("{} is decribed as: '{:#?}'", input_key, &value_data[0].value),
            )
            .await;
    } else {
        msg.reply(
            ctx,
            &format!(
                "There was a problem looking up the value for {}",
                &input_key
            ),
        )
        .await?;

        return Ok(());
    }
    Ok(())
}
