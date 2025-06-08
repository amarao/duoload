use crate::duocards::models::VocabularyCard;
use crate::error::Result;
use std::path::Path;

pub mod anki;
pub mod json;

pub trait OutputBuilder: Send + Sync {
    fn add_note(&mut self, card: VocabularyCard) -> Result<bool>;
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}
