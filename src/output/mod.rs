use crate::duocards::models::VocabularyCard;
use crate::error::Result;
use std::io::Write;

pub mod anki;
pub mod json;

pub trait OutputBuilder: Send + Sync {
    fn add_note(&mut self, card: VocabularyCard) -> Result<bool>;
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
}
