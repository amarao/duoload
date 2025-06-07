use crate::error::Result;
use crate::duocards::models::{DuocardsResponse, VocabularyCard};

mod client;
pub mod models;
pub mod auth;

pub use client::DuocardsClient;
pub use models::{Card, CardEdge, CardConnection, PageInfo, Deck, ResponseData, Extensions, LearningStatus};
pub use auth::AuthError;

#[async_trait::async_trait]
pub trait DuocardsClientTrait: Send + Sync {
    async fn fetch_page(&self, deck_id: &str, cursor: Option<String>) -> Result<DuocardsResponse>;
    fn convert_to_vocabulary_cards(&self, response: &DuocardsResponse) -> Vec<VocabularyCard>;
    fn validate_deck_id(&self, deck_id: &str) -> Result<()>;
}
