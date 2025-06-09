use crate::duocards::models::VocabularyCard;
use crate::error::Result;
use std::io::Write;
use std::path::Path;

pub mod anki;
pub mod json;

/// Output destination for builders
pub enum OutputDestination<'a> {
    /// Write to a generic writer (stdout, buffer, etc)
    Writer(&'a mut (dyn Write + 'a)),
    /// Write to a file at the given path
    File(&'a Path),
}

pub trait OutputBuilder: Send + Sync {
    fn add_note(&mut self, card: VocabularyCard) -> Result<bool>;
    fn write(&self, dest: OutputDestination<'_>) -> Result<()>;
}
