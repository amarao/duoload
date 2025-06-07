use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DuocardsResponse {
    pub data: Vec<VocabularyCard>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Deserialize)]
pub struct VocabularyCard {
    pub word: String,
    pub translation: String,
    pub example: Option<String>,
    pub status: LearningStatus,
}

#[derive(Debug, Deserialize)]
pub enum LearningStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "learning")]
    Learning,
    #[serde(rename = "known")]
    Known,
}

#[derive(Debug, Deserialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub has_next: bool,
}
