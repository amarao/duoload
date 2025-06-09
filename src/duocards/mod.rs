use crate::duocards::models::{DuocardsResponse, VocabularyCard};
use crate::error::Result;

pub mod client;
pub mod deck;
pub mod models;

pub use client::DuocardsClient;

#[async_trait::async_trait]
pub trait DuocardsClientTrait: Send + Sync {
    async fn fetch_page(&self, deck_id: &str, cursor: Option<String>) -> Result<DuocardsResponse>;
    fn convert_to_vocabulary_cards(&self, response: &DuocardsResponse) -> Vec<VocabularyCard>;
}
