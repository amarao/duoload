use crate::error::Result;
use crate::duocards::models::{DuocardsResponse, VocabularyCard};

mod client;
pub mod models;
pub mod deck;

pub use client::DuocardsClient;

#[async_trait::async_trait]
pub trait DuocardsClientTrait: Send + Sync {
    async fn fetch_page(&self, deck_id: &str, cursor: Option<String>) -> Result<DuocardsResponse>;
    fn convert_to_vocabulary_cards(&self, response: &DuocardsResponse) -> Vec<VocabularyCard>;
}
