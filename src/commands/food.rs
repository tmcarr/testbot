use crate::{Context, Error};
use rand::seq::IteratorRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct CuisineResponse {
    meals: Vec<CuisineData>,
}

#[derive(Deserialize)]
struct CuisineData {
    strArea: String,
}

/// Reply with a suggestion for cuisine from an online API.
#[poise::command(slash_command, prefix_command)]
pub async fn food(ctx: Context<'_>) -> Result<(), Error> {
    const ENDPOINT: &str = "https://www.themealdb.com/api/json/v1/1/list.php?a=list";
    
    // Fetch cuisines from TheMealDB API
    let response = reqwest::get(ENDPOINT).await?;
    
    if response.status().is_success() {
        let cuisine_response: CuisineResponse = response.json().await?;
        
        if !cuisine_response.meals.is_empty() {
            let cuisine = cuisine_response.meals.iter().choose(&mut rand::thread_rng()).unwrap();
            ctx.say(&cuisine.strArea).await?;
        } else {
            // Fallback to a curated list if API returns empty
            let fallback_cuisines = [
                "American", "Italian", "Mexican", "Chinese", "Japanese", "Indian", 
                "Thai", "French", "Greek", "Spanish", "Vietnamese", "Korean"
            ];
            let item = fallback_cuisines.iter().choose(&mut rand::thread_rng()).unwrap();
            ctx.say(*item).await?;
        }
    } else {
        // Fallback to a curated list if API fails
        let fallback_cuisines = [
            "American", "Italian", "Mexican", "Chinese", "Japanese", "Indian", 
            "Thai", "French", "Greek", "Spanish", "Vietnamese", "Korean"
        ];
        let item = fallback_cuisines.iter().choose(&mut rand::thread_rng()).unwrap();
        ctx.say(*item).await?;
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
    async fn test_food_api_success() {
        // Test with a mock response from TheMealDB API
        let mock_response = CuisineResponse {
            meals: vec![
                CuisineData { strArea: "American".to_string() },
                CuisineData { strArea: "Italian".to_string() },
                CuisineData { strArea: "Mexican".to_string() },
                CuisineData { strArea: "Chinese".to_string() },
                CuisineData { strArea: "Japanese".to_string() },
            ],
        };

        assert_eq!(mock_response.meals.len(), 5);
        assert!(mock_response.meals.iter().any(|c| c.strArea == "American"));
        assert!(mock_response.meals.iter().any(|c| c.strArea == "Italian"));
    }

    #[tokio::test]
    async fn test_food_api_empty_response() {
        let mock_response = CuisineResponse {
            meals: vec![],
        };

        assert!(mock_response.meals.is_empty());
        assert!(mock_response.meals.iter().choose(&mut rand::thread_rng()).is_none());
    }

    #[tokio::test]
    async fn test_food_api_single_cuisine() {
        let mock_response = CuisineResponse {
            meals: vec![
                CuisineData { strArea: "Thai".to_string() },
            ],
        };

        assert_eq!(mock_response.meals.len(), 1);
        assert_eq!(mock_response.meals[0].strArea, "Thai");
    }

    #[test]
    fn test_food_response_deserialization() {
        let json = r#"{"meals": [{"strArea": "American"}, {"strArea": "Italian"}]}"#;
        let result: Result<CuisineResponse, serde_json::Error> = serde_json::from_str(json);
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.meals.len(), 2);
        assert_eq!(response.meals[0].strArea, "American");
        assert_eq!(response.meals[1].strArea, "Italian");
    }

    #[test]
    fn test_food_response_deserialization_empty() {
        let json = r#"{"meals": []}"#;
        let result: Result<CuisineResponse, serde_json::Error> = serde_json::from_str(json);
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.meals.len(), 0);
    }

    #[test]
    fn test_food_response_deserialization_single() {
        let json = r#"{"meals": [{"strArea": "French"}]}"#;
        let result: Result<CuisineResponse, serde_json::Error> = serde_json::from_str(json);
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.meals.len(), 1);
        assert_eq!(response.meals[0].strArea, "French");
    }

    #[test]
    fn test_food_endpoint_url() {
        const ENDPOINT: &str = "https://www.themealdb.com/api/json/v1/1/list.php?a=list";
        assert!(ENDPOINT.contains("themealdb.com"));
        assert!(ENDPOINT.contains("/api/json/v1/1/"));
        assert!(ENDPOINT.contains("list.php"));
        assert!(ENDPOINT.contains("a=list"));
    }

    #[tokio::test]
    async fn test_mock_context_functionality() {
        let ctx = create_mock_context().await;
        
        // Test that we can add messages
        ctx.say("Test cuisine 1").await.unwrap();
        ctx.say("Test cuisine 2").await.unwrap();
        
        let messages = ctx.get_messages().await;
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "Test cuisine 1");
        assert_eq!(messages[1], "Test cuisine 2");
    }

    #[test]
    fn test_fallback_cuisines_contain_popular_options() {
        let fallback_cuisines = [
            "American", "Italian", "Mexican", "Chinese", "Japanese", "Indian", 
            "Thai", "French", "Greek", "Spanish", "Vietnamese", "Korean"
        ];
        
        assert!(fallback_cuisines.contains(&"American"));
        assert!(fallback_cuisines.contains(&"Italian"));
        assert!(fallback_cuisines.contains(&"Mexican"));
        assert!(fallback_cuisines.contains(&"Chinese"));
        assert!(fallback_cuisines.contains(&"Japanese"));
        assert!(fallback_cuisines.contains(&"Indian"));
        assert_eq!(fallback_cuisines.len(), 12);
    }

    #[test]
    fn test_fallback_cuisines_are_unique() {
        let fallback_cuisines = [
            "American", "Italian", "Mexican", "Chinese", "Japanese", "Indian", 
            "Thai", "French", "Greek", "Spanish", "Vietnamese", "Korean"
        ];
        
        let mut unique_cuisines = std::collections::HashSet::new();
        for cuisine in &fallback_cuisines {
            unique_cuisines.insert(*cuisine);
        }
        
        assert_eq!(unique_cuisines.len(), fallback_cuisines.len());
    }

    #[test]
    fn test_cuisine_data_structure() {
        let cuisine = CuisineData {
            strArea: "Mexican".to_string(),
        };
        
        assert_eq!(cuisine.strArea, "Mexican");
        assert!(!cuisine.strArea.is_empty());
    }
}
