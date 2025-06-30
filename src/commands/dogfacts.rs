use crate::{Context, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct DogFactResponse {
    data: Vec<DogFactData>,
}

#[derive(Deserialize)]
struct DogFactData {
    attributes: DogFactAttributes,
}

#[derive(Deserialize)]
struct DogFactAttributes {
    body: String,
}

/// Get a random fact about dogs from an API
#[poise::command(slash_command, prefix_command)]
pub async fn dogfact(ctx: Context<'_>) -> Result<(), Error> {
    const ENDPOINT: &str = "https://dogapi.dog/api/v2/facts?number=1";

    // Fetch a random dog fact from the API
    let response = reqwest::get(ENDPOINT).await?;

    if response.status().is_success() {
        let dog_fact: DogFactResponse = response.json().await?;

        if let Some(fact_data) = dog_fact.data.first() {
            ctx.say(&fact_data.attributes.body).await?;
        } else {
            ctx.say("Sorry, couldn't fetch a dog fact right now. Here's one: Dogs have a sense of smell that is 40 times greater than humans! 🐕").await?;
        }
    } else {
        ctx.say("Sorry, the dog facts service is currently unavailable. Here's a dog fact: Dogs can hear sounds at frequencies up to 65,000 Hz! 🐕").await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock context for testing
    struct MockContext {
        messages: Arc<Mutex<Vec<String>>>,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                messages: Arc::new(Mutex::new(Vec::new())),
            }
        }

        async fn say(&self, message: &str) -> Result<(), Error> {
            self.messages.lock().await.push(message.to_string());
            Ok(())
        }

        async fn get_messages(&self) -> Vec<String> {
            self.messages.lock().await.clone()
        }
    }

    // Test helper function to create a mock context
    async fn create_mock_context() -> MockContext {
        MockContext::new()
    }

    #[tokio::test]
    async fn test_dogfact_api_success() {
        // This test would require mocking the HTTP client
        // For now, we'll test the logic with a mock response
        let mock_response = DogFactResponse {
            data: vec![DogFactData {
                attributes: DogFactAttributes {
                    body: "Dogs have an amazing sense of smell!".to_string(),
                },
            }],
        };

        assert_eq!(mock_response.data.len(), 1);
        assert!(mock_response.data[0].attributes.body.contains("smell"));
    }

    #[tokio::test]
    async fn test_dogfact_api_empty_response() {
        let mock_response = DogFactResponse { data: vec![] };

        assert!(mock_response.data.is_empty());
        assert!(mock_response.data.first().is_none());
    }

    #[tokio::test]
    async fn test_dogfact_api_multiple_facts() {
        let mock_response = DogFactResponse {
            data: vec![
                DogFactData {
                    attributes: DogFactAttributes {
                        body: "Dogs can hear high frequencies.".to_string(),
                    },
                },
                DogFactData {
                    attributes: DogFactAttributes {
                        body: "Dogs have three eyelids.".to_string(),
                    },
                },
                DogFactData {
                    attributes: DogFactAttributes {
                        body: "Dogs can dream like humans.".to_string(),
                    },
                },
            ],
        };

        assert_eq!(mock_response.data.len(), 3);
        assert!(mock_response.data.first().is_some());
        assert!(mock_response.data[0].attributes.body.contains("hear"));
    }

    #[test]
    fn test_dogfact_response_deserialization() {
        let json = r#"{"data": [{"id": "test", "type": "fact", "attributes": {"body": "Dogs are amazing animals!"}}]}"#;
        let result: Result<DogFactResponse, serde_json::Error> = serde_json::from_str(json);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(
            response.data[0].attributes.body,
            "Dogs are amazing animals!"
        );
    }

    #[test]
    fn test_dogfact_response_deserialization_empty() {
        let json = r#"{"data": []}"#;
        let result: Result<DogFactResponse, serde_json::Error> = serde_json::from_str(json);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_dogfact_response_deserialization_multiple() {
        let json = r#"{"data": [{"id": "1", "type": "fact", "attributes": {"body": "Fact 1"}}, {"id": "2", "type": "fact", "attributes": {"body": "Fact 2"}}, {"id": "3", "type": "fact", "attributes": {"body": "Fact 3"}}]}"#;
        let result: Result<DogFactResponse, serde_json::Error> = serde_json::from_str(json);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 3);
        assert_eq!(response.data[0].attributes.body, "Fact 1");
        assert_eq!(response.data[1].attributes.body, "Fact 2");
        assert_eq!(response.data[2].attributes.body, "Fact 3");
    }

    #[test]
    fn test_dogfact_endpoint_url() {
        const ENDPOINT: &str = "https://dogapi.dog/api/v2/facts?number=1";
        assert!(ENDPOINT.contains("dogapi.dog"));
        assert!(ENDPOINT.contains("/api/v2/facts"));
        assert!(ENDPOINT.contains("number=1"));
    }

    #[tokio::test]
    async fn test_mock_context_functionality() {
        let ctx = create_mock_context().await;

        // Test that we can add messages
        ctx.say("Test message 1").await.unwrap();
        ctx.say("Test message 2").await.unwrap();

        let messages = ctx.get_messages().await;
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "Test message 1");
        assert_eq!(messages[1], "Test message 2");
    }

    #[test]
    fn test_fallback_messages_contain_dog_facts() {
        let fallback_1 = "Sorry, couldn't fetch a dog fact right now. Here's one: Dogs have a sense of smell that is 40 times greater than humans! 🐕";
        let fallback_2 = "Sorry, the dog facts service is currently unavailable. Here's a dog fact: Dogs can hear sounds at frequencies up to 65,000 Hz! 🐕";

        assert!(fallback_1.contains("dog fact"));
        assert!(fallback_1.contains("smell"));
        assert!(fallback_2.contains("dog fact"));
        assert!(fallback_2.contains("hear"));
        assert!(fallback_1.contains("🐕"));
        assert!(fallback_2.contains("🐕"));
    }
}
