use futures::future::FutureExt;
use rand::seq::SliceRandom;
use serenity::model::interactions::application_command::ApplicationCommandInteractionData;

pub fn botsnack(
    _: &ApplicationCommandInteractionData,
) -> crate::BoxFuture<'static, Option<String>> {
    async move {
        let responses = vec!["Yum!", "*cronch*", "MOAR", "*Smiles*", "Nice."];
        let response = *responses.choose(&mut rand::thread_rng()).unwrap();

        Some(response.to_owned())
    }
    .boxed()
}
