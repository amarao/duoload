use crate::error::Result;
use crate::duocards::models::VocabularyCard;
use std::path::Path;

pub mod deck;
pub mod note;

#[cfg(test)]
mod deck_test;
#[cfg(test)]
mod note_test;

pub use deck::AnkiPackageBuilder;
pub use note::VocabularyNote;

pub trait AnkiPackageBuilderTrait: Send + Sync {
    fn add_note(&mut self, card: VocabularyCard) -> Result<bool>;
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}
