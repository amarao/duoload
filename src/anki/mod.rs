use crate::duocards::models::VocabularyCard;
use crate::error::Result;
use std::path::Path;

pub mod deck;
pub mod note;

pub use deck::AnkiPackageBuilder;

pub trait AnkiPackageBuilderTrait: Send + Sync {
    fn add_note(&mut self, card: VocabularyCard) -> Result<bool>;
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

#[cfg(test)]
mod deck_test;
#[cfg(test)]
mod note_test;
