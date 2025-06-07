use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuocardsResponse {
    pub data: ResponseData,
    pub extensions: Extensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData {
    pub node: Deck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub __typename: String,
    pub cards: CardConnection,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardConnection {
    pub edges: Vec<CardEdge>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardEdge {
    pub node: Card,
    pub cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
    pub hint: Option<String>,
    pub waiting: Option<Value>,
    #[serde(rename = "knownCount")]
    pub known_count: i32,
    pub svg: Option<CardImage>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardImage {
    #[serde(rename = "flatId")]
    pub flat_id: Option<String>,
    pub url: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extensions {
    #[serde(rename = "releaseId")]
    pub release_id: Option<String>,
}

// Our internal representation of a vocabulary card
#[derive(Debug, Clone)]
pub struct VocabularyCard {
    pub word: String,
    pub translation: String,
    pub example: Option<String>,
    pub status: LearningStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LearningStatus {
    New,
    Learning,
    Known,
}

impl From<Card> for VocabularyCard {
    fn from(card: Card) -> Self {
        // Convert known_count to LearningStatus
        let status = if card.known_count >= 5 {
            LearningStatus::Known
        } else if card.known_count > 0 {
            LearningStatus::Learning
        } else {
            LearningStatus::New
        };

        Self {
            word: card.front,
            translation: card.back,
            example: card.hint,
            status,
        }
    }
}

// GraphQL query types
#[derive(Debug, Serialize)]
pub struct CardsQuery {
    pub query: String,
    pub variables: CardsQueryVariables,
}

#[derive(Debug, Serialize)]
pub struct CardsQueryVariables {
    pub count: i32,
    pub cursor: Option<String>,
    #[serde(rename = "deckId")]
    pub deck_id: String,
    pub search: String,
    #[serde(rename = "cardState")]
    pub card_state: Option<String>,
}

impl CardsQuery {
    pub fn new(deck_id: &str, count: i32, cursor: Option<String>) -> Self {
        Self {
            query: include_str!("../../internal_docs/duocards/query.graphql").to_string(),
            variables: CardsQueryVariables {
                count,
                cursor,
                deck_id: deck_id.to_string(),
                search: String::new(),
                card_state: None,
            },
        }
    }
}
