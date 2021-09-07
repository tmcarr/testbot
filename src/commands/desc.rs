use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::*;
use crate::schema::descriptions::dsl::*;
use crate::PostgresClient;
use crate::{SlashCommand, SlashCommandOption};
use diesel::r2d2::ManageConnection;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

// TODO: #[aliases("set")]
async fn describe(ctx: &Context, input_key: &str, input_value: &str) -> CommandResult<String> {
    let data = ctx.data.read().await;

    if let Some(dbclient) = data.get::<PostgresClient>() {
        let connection = dbclient.connect().expect("Could not connect to Postgres");
        let description = NewDescription {
            key: input_key,
            value: input_value,
        };
        diesel::insert_into(descriptions)
            .values(&description)
            .on_conflict(key)
            .do_update()
            .set(&description)
            .execute(&connection)?;
        Ok(format!("Defining {} as: '{}'", input_key, input_value))
    } else {
        Ok(format!(
            "There was a problem reading from the databse. Failed to define {} as '{}'",
            input_key, input_value
        ))
    }
}

async fn handle_describe(
    ctx: &Context,
    data: &ApplicationCommandInteractionData,
) -> CommandResult<String> {
    let arguments = super::get_string_arguments(data);

    match (arguments.get("key"), arguments.get("value")) {
        (Some(&input_key), Some(&input_value)) => describe(ctx, input_key, input_value).await,
        _ => Err(Box::new(super::CommandError::OptionMissing)),
    }
}

make_slash_command_handler!(DescribeHandler, handle_describe);

lazy_static::lazy_static! {
    pub(crate) static ref DESCRIBE_COMMAND: SlashCommand = SlashCommand {
        description: "Sets the stored definition for a word",
        options: vec![
            SlashCommandOption {
                name: "key".to_string(),
                description: "Key".to_string(),
                required: true,
            },
            SlashCommandOption {
                name: "value".to_string(),
                description: "Its definition".to_string(),
                required: true,
            },
        ],
        handler: &DescribeHandler,
    };
}

// TODO: #[aliases("show", "get")]
async fn define(ctx: &Context, input_key: &str) -> CommandResult<String> {
    let data = ctx.data.read().await;

    if let Some(dbclient) = data.get::<PostgresClient>() {
        let connection = dbclient.connect().expect("Could not connect to Postgres");

        // Do DB Read here
        let value_data = descriptions
            .filter(key.eq(&input_key))
            .load::<Description>(&connection)
            .expect("Error loading results.");

        Ok(format!(
            "{} is decribed as: '{}'",
            input_key, &value_data[0].value
        ))
    } else {
        Ok(format!(
            "There was a problem looking up the value for {}",
            &input_key
        ))
    }
}

async fn handle_define(
    ctx: &Context,
    data: &ApplicationCommandInteractionData,
) -> CommandResult<String> {
    match super::get_string_arguments(data).get("key") {
        Some(&input_key) => define(ctx, input_key).await,
        None => Err(Box::new(super::CommandError::OptionMissing)),
    }
}

make_slash_command_handler!(DefineHandler, handle_define);

lazy_static::lazy_static! {
    pub(crate) static ref DEFINE_COMMAND: SlashCommand = SlashCommand {
        description: "Retrieves the stored definition for a word",
        options: vec![
            SlashCommandOption {
                name: "key".to_string(),
                description: "Key".to_string(),
                required: true,
            },
        ],
        handler: &DefineHandler,
    };
}
