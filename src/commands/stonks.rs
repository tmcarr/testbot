use crate::{Context, Error};

/// Display the Finviz graph for a given ticker.
#[poise::command(slash_command, prefix_command)]
pub async fn stonks(
    ctx: Context<'_>,
    #[description = "Stock ticker symbol"] ticker: String,
) -> Result<(), Error> {
    let url = format!(
        "https://www.finviz.com/chart.ashx?t={}&ty=c&ta=1&p=d&s=l",
        &ticker
    );
    ctx.say(url).await?;
    Ok(())
}
