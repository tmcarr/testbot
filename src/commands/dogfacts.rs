use serde::Deserialize;
use crate::{Context, Error};

#[derive(Deserialize)]
struct DogFactResponse {
    facts: Vec<String>,
}

/// Get a random fact about dogs from an API
#[poise::command(slash_command, prefix_command)]
pub async fn dogfact(ctx: Context<'_>) -> Result<(), Error> {
    const ENDPOINT: &str = "https://dogapi.dog/api/v2/facts?number=1";
    
    // Fetch a random dog fact from the API
    let response = reqwest::get(ENDPOINT).await?;
    
    if response.status().is_success() {
        let dog_fact: DogFactResponse = response.json().await?;
        
        if let Some(fact) = dog_fact.facts.first() {
            ctx.say(fact).await?;
        } else {
            ctx.say("Sorry, couldn't fetch a dog fact right now. Here's one: Dogs have a sense of smell that is 40 times greater than humans! 🐕").await?;
        }
    } else {
        ctx.say("Sorry, the dog facts service is currently unavailable. Here's a dog fact: Dogs can hear sounds at frequencies up to 65,000 Hz! 🐕").await?;
    }

    Ok(())
} 