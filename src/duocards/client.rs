use crate::duocards::deck;
use crate::duocards::{
    DuocardsClientTrait,
    models::{CardsQuery, DuocardsResponse, VocabularyCard},
};
use crate::error::{DuoloadError, Result};
use async_trait::async_trait;
use reqwest::{
    Client,
    header::{ACCEPT_ENCODING, CONTENT_TYPE, HeaderMap, HeaderValue},
};
use std::time::Duration;

const BASE_URL: &str = "https://api.duocards.com/graphql";
const USER_AGENT: &str = "duoload/1.0";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_PAGE_SIZE: i32 = 100;

#[derive(Debug, Clone)]
pub struct DuocardsClient {
    client: Client,
    base_url: String,
}

impl DuocardsClient {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        // headers.insert(ORIGIN, HeaderValue::from_static("https://app.duocards.com"));
        // headers.insert(REFERER, HeaderValue::from_static("https://app.duocards.com/"));
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br, zstd"),
        );

        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(DEFAULT_TIMEOUT)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: BASE_URL.to_string(),
        })
    }

    pub async fn fetch_page(
        &self,
        deck_id: &str,
        cursor: Option<String>,
    ) -> Result<DuocardsResponse> {
        // Validate deck ID before making the request
        deck::validate_deck_id(deck_id)?;

        let query = CardsQuery::new(deck_id, DEFAULT_PAGE_SIZE, cursor);

        let response = self.client.post(&self.base_url).json(&query).send().await?;

        if !response.status().is_success() {
            return Err(DuoloadError::Api(format!(
                "API request failed with status {}: {}",
                response.status(),
                response.text().await?
            )));
        }

        let response: DuocardsResponse = response.json().await?;
        Ok(response)
    }

    // Helper method to convert API response to our internal card format
    pub fn convert_to_vocabulary_cards(&self, response: &DuocardsResponse) -> Vec<VocabularyCard> {
        response
            .data
            .node
            .cards
            .edges
            .iter()
            .map(|edge| VocabularyCard::from(edge.node.clone()))
            .collect()
    }
}

#[async_trait]
impl DuocardsClientTrait for DuocardsClient {
    async fn fetch_page(&self, deck_id: &str, cursor: Option<String>) -> Result<DuocardsResponse> {
        self.fetch_page(deck_id, cursor).await
    }

    fn convert_to_vocabulary_cards(&self, response: &DuocardsResponse) -> Vec<VocabularyCard> {
        self.convert_to_vocabulary_cards(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::LearningStatus;
    use mockito::Server;
    use serde_json::json;
    use tokio_test::block_on;

    // Valid test deck ID (base64 encoded "Deck:46f2b9ed-abf3-4bd8-a054-68dfa4a4203e")
    const TEST_DECK_ID: &str = "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=";

    fn create_mock_response() -> serde_json::Value {
        json!({
            "data": {
                "node": {
                    "__typename": "Deck",
                    "cards": {
                        "edges": [
                            {
                                "node": {
                                    "id": "test-id",
                                    "front": "hello",
                                    "back": "hola",
                                    "hint": "Hello, world!",
                                    "waiting": null,
                                    "knownCount": 5,
                                    "svg": null,
                                    "__typename": "Card"
                                },
                                "cursor": "0"
                            }
                        ],
                        "pageInfo": {
                            "endCursor": "0",
                            "hasNextPage": true
                        }
                    },
                    "id": TEST_DECK_ID
                }
            },
            "extensions": {
                "releaseId": "2025-06-04T14:06:15.707Z"
            }
        })
    }

    #[test]
    fn test_fetch_page() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/graphql")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_response().to_string())
            .create();

        // Override the base URL to use the mock server
        let mut client = DuocardsClient::new().unwrap();
        client.base_url = server.url() + "/graphql";

        let response = block_on(client.fetch_page(TEST_DECK_ID, None)).unwrap();

        mock.assert();
        assert_eq!(response.data.node.id, TEST_DECK_ID);
        assert_eq!(response.data.node.cards.edges.len(), 1);
        assert_eq!(response.data.node.cards.edges[0].node.front, "hello");
        assert_eq!(response.data.node.cards.edges[0].node.back, "hola");
        assert_eq!(response.data.node.cards.edges[0].node.known_count, 5);
        assert_eq!(
            response.data.node.cards.page_info.end_cursor,
            Some("0".to_string())
        );
        assert!(response.data.node.cards.page_info.has_next_page);
    }

    #[test]
    fn test_convert_to_vocabulary_cards() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/graphql")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_response().to_string())
            .create();

        // Override the base URL to use the mock server
        let mut client = DuocardsClient::new().unwrap();
        client.base_url = server.url() + "/graphql";

        let response = block_on(client.fetch_page(TEST_DECK_ID, None)).unwrap();
        let cards = client.convert_to_vocabulary_cards(&response);

        mock.assert();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].word, "hello");
        assert_eq!(cards[0].translation, "hola");
        assert_eq!(cards[0].example, Some("Hello, world!".to_string()));
        assert!(matches!(cards[0].status, LearningStatus::Known));
    }
}
