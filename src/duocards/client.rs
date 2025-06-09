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
    pub base_url: String,
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
