use genanki_rs::{Model, Note};
use crate::duocards::models::VocabularyCard;
use anyhow::Result;

pub struct VocabularyNote {
    pub word: String,
    pub translation: String,
    pub example: String,
    pub tags: Vec<String>,
}

impl From<VocabularyCard> for VocabularyNote {
    fn from(_card: VocabularyCard) -> Self {
        unimplemented!("VocabularyNote::from")
    }
}

impl VocabularyNote {
    pub fn to_anki_note(&self, _model: &Model) -> Result<Note> {
        unimplemented!("VocabularyNote::to_anki_note")
    }
}
