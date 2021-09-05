use futures::future::BoxFuture;
use futures::FutureExt;
use rand::seq::IteratorRandom;
use serenity::model::prelude::application_command::*;

// TODO: #[aliases("rand")]

pub fn handler(data: &ApplicationCommandInteractionData) -> BoxFuture<'static, Option<String>> {
    let arguments = super::get_string_arguments(data);

    let thing = arguments.values().choose(&mut rand::thread_rng());

    let response = match thing {
        Some(&choice) => choice.to_owned(),
        _ => "Why u no args?!".to_owned(),
    };

    async move { Some(response) }.boxed()
}
