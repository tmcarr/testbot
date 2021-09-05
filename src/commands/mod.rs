// The make_slash_command_handler macro will be available to all of the child modules, thanks to the
// historical "textual scope" that applies to macros.  #[macro_export] makes scoping rules more
// normal, but forcefully makes the macro pub and that is even worse.
macro_rules! make_slash_command_handler {
    ($struct_name:ident, $function:ident) => {
        struct $struct_name;

        #[crate::async_trait]
        impl crate::SlashCommandHandler for $struct_name {
            async fn handle(
                &self,
                ctx: &$crate::Context,
                data: &$crate::ApplicationCommandInteractionData,
            ) -> CommandResult<String> {
                $function(ctx, data).await
            }
        }
    };
}

pub mod advice;
pub mod ball;
pub mod botsnack;
pub mod desc;
pub mod drink;
pub mod food;
pub mod github;
pub mod owner;
pub mod pingpong;
pub mod random;
pub mod stonks;

fn get_string_arguments(
    data: &crate::ApplicationCommandInteractionData,
) -> std::collections::HashMap<&str, &str> {
    data.options
        .iter()
        // We should only ever get a JSON string from the Discord API, but if they don't uphold
        // that, then we'll ignore the value completely.
        .filter_map(|o| {
            o.value
                .as_ref()
                .and_then(|v| v.as_str())
                .map(|v| (o.name.as_str(), v))
        })
        .collect()
}
