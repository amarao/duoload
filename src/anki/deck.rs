use std::collections::HashSet;
use std::path::Path;
use genanki_rs::{Deck, Model, Note};
use anyhow::Result;
use crate::duocards::models::VocabularyCard;
use super::note::VocabularyNote;

pub struct AnkiPackageBuilder {
    _deck: Deck,
    _model: Model,
    _existing_words: Vec<String>,
}

impl AnkiPackageBuilder {
    pub fn new(_deck_name: &str) -> Self {
        unimplemented!("AnkiPackageBuilder::new")
    }
    
    pub fn add_note(&mut self, _vocab_card: VocabularyCard) -> Result<bool> {
        unimplemented!("AnkiPackageBuilder::add_note")
    }
    
    pub fn write_to_file<P: AsRef<Path>>(&self, _path: P) -> Result<()> {
        unimplemented!("AnkiPackageBuilder::write_to_file")
    }
}
