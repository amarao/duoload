//! Anki package builder module for creating and managing Anki decks.
//! 
//! This module provides functionality to create Anki packages (.apkg files)
//! from vocabulary cards, handling duplicate detection and proper note creation.

use std::collections::HashSet;
use std::path::Path;
use genanki_rs::Deck;
use anyhow::Result as AnyhowResult;
use crate::duocards::models::VocabularyCard;
use super::note::{VocabularyNote, create_vocabulary_model};
use crate::error::{Result, DuoloadError};
use crate::anki::AnkiPackageBuilderTrait;

/// Builder for creating Anki packages from vocabulary cards.
/// 
/// This struct manages the creation of an Anki package, handling:
/// - Deck creation and configuration
/// - Note addition with duplicate detection
/// - Package file generation
pub struct AnkiPackageBuilder {
    pub deck: Deck,
    pub model: genanki_rs::Model,
    pub existing_words: HashSet<String>,
}

impl AnkiPackageBuilder {
    /// Creates a new Anki package builder with the specified deck name.
    /// 
    /// # Arguments
    /// 
    /// * `deck_name` - The name of the deck to create
    /// 
    /// # Returns
    /// 
    /// A new AnkiPackageBuilder instance configured with the specified deck name.
    pub fn new(deck_name: &str) -> Self {
        let model = create_vocabulary_model();
        let deck = Deck::new(
            2059400110, // Deck ID - fixed for consistency
            deck_name,
            "Vocabulary imported from Duocards"
        );
        
        Self {
            deck,
            model,
            existing_words: HashSet::new(),
        }
    }
    
    /// Adds a vocabulary card to the deck.
    /// 
    /// This method converts the vocabulary card to an Anki note and adds it to the deck.
    /// Duplicate words are detected and skipped.
    /// 
    /// # Arguments
    /// 
    /// * `vocab_card` - The vocabulary card to add
    /// 
    /// # Returns
    /// 
    /// A Result containing a boolean indicating whether the card was added (true)
    /// or was a duplicate (false), or an error if note creation fails.
    pub fn add_note(&mut self, vocab_card: VocabularyCard) -> AnyhowResult<bool> {
        // Check for duplicates before moving the card
        if self.existing_words.contains(&vocab_card.word) {
            return Ok(false); // Duplicate
        }
        
        // Clone the word before moving the card
        let word = vocab_card.word.clone();
        
        // Create and add the note
        let note = VocabularyNote::from(vocab_card).to_anki_note(&self.model)?;
        self.deck.add_note(note);
        self.existing_words.insert(word);
        Ok(true)
    }
    
    /// Writes the deck to an Anki package file.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path where the .apkg file should be written
    /// 
    /// # Returns
    /// 
    /// A Result indicating success or failure of the write operation.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> AnyhowResult<()> {
        self.deck.write_to_file(path.as_ref().to_str().ok_or_else(|| {
            anyhow::anyhow!("Invalid file path: {:?}", path.as_ref())
        })?)
        .map_err(|e| anyhow::anyhow!("Failed to write Anki package: {}", e))
    }
}

impl AnkiPackageBuilderTrait for AnkiPackageBuilder {
    fn add_note(&mut self, card: VocabularyCard) -> std::result::Result<bool, DuoloadError> {
        self.add_note(card).map_err(DuoloadError::from)
    }

    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), DuoloadError> {
        self.write_to_file(path).map_err(DuoloadError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::{VocabularyCard, LearningStatus};
    use tempfile::NamedTempFile;
    use std::fs;

    fn create_test_card(word: &str, translation: &str, example: Option<&str>, status: LearningStatus) -> VocabularyCard {
        VocabularyCard {
            word: word.to_string(),
            translation: translation.to_string(),
            example: example.map(|s| s.to_string()),
            status,
        }
    }

    #[test]
    fn test_new_builder() {
        let mut builder = AnkiPackageBuilder::new("Test Deck");
        assert!(builder.existing_words.is_empty());
        
        // Verify we can add a note to a new builder
        let card = create_test_card(
            "hello",
            "hola",
            Some("Hello, world!"),
            LearningStatus::New,
        );
        assert!(builder.add_note(card).unwrap());
    }

    #[test]
    fn test_add_note() {
        let mut builder = AnkiPackageBuilder::new("Test Deck");
        
        // Add first note
        let card1 = create_test_card(
            "hello",
            "hola",
            Some("Hello, world!"),
            LearningStatus::New,
        );
        assert!(builder.add_note(card1).unwrap());
        
        // Add duplicate note
        let card2 = create_test_card(
            "hello",
            "hola",
            Some("Hello again!"),
            LearningStatus::Learning,
        );
        assert!(!builder.add_note(card2).unwrap());
        
        // Add different note
        let card3 = create_test_card(
            "goodbye",
            "adiós",
            Some("Goodbye, world!"),
            LearningStatus::Known,
        );
        assert!(builder.add_note(card3).unwrap());
    }

    #[test]
    fn test_write_to_file() {
        let mut builder = AnkiPackageBuilder::new("Test Deck");
        
        // Add some notes
        let card1 = create_test_card(
            "hello",
            "hola",
            Some("Hello, world!"),
            LearningStatus::New,
        );
        builder.add_note(card1).unwrap();
        
        let card2 = create_test_card(
            "goodbye",
            "adiós",
            Some("Goodbye, world!"),
            LearningStatus::Known,
        );
        builder.add_note(card2).unwrap();
        
        // Write to temporary file
        let temp_file = NamedTempFile::new().unwrap();
        builder.write_to_file(&temp_file).unwrap();
        
        // Verify file exists and has content
        let metadata = fs::metadata(&temp_file).unwrap();
        assert!(metadata.len() > 0);
    }

    #[test]
    fn test_write_to_file_invalid_path() {
        let builder = AnkiPackageBuilder::new("Test Deck");
        
        // Try to write to an invalid path
        let result = builder.write_to_file("/invalid/path/with/nulls/\0");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_deck() {
        let builder = AnkiPackageBuilder::new("Empty Deck");
        let temp_file = NamedTempFile::new().unwrap();
        
        // Should still be able to write an empty deck
        builder.write_to_file(&temp_file).unwrap();
        
        // Verify file exists
        let metadata = fs::metadata(&temp_file).unwrap();
        assert!(metadata.len() > 0);
    }
}
