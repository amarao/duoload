//! JSON generator module for creating JSON output files from vocabulary cards.
//!
//! This module provides functionality to create JSON files containing vocabulary cards,
//! handling duplicate detection and proper JSON serialization.

use crate::duocards::models::{VocabularyCard, LearningStatus};
use crate::error::Result;
use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

/// Trait defining the interface for JSON generation
pub trait JsonGeneratorTrait: Send + Sync {
    /// Adds a vocabulary card to the generator
    /// Returns true if the card was added, false if it was a duplicate
    fn add_card(&mut self, card: VocabularyCard) -> Result<bool>;

    /// Writes the generated JSON to a file
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

/// Builder for creating JSON files from vocabulary cards
///
/// This struct manages the creation of a JSON file, handling:
/// - Card collection and duplicate detection
/// - JSON serialization
/// - File writing
pub struct JsonGenerator {
    cards: Vec<VocabularyCard>,
    existing_words: HashSet<String>,
    duplicate_count: usize,
    start_time: Instant,
}

impl Default for JsonGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonGenerator {
    /// Creates a new JSON generator
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            existing_words: HashSet::new(),
            duplicate_count: 0,
            start_time: Instant::now(),
        }
    }

    /// Returns true if the word is a duplicate (has been added before)
    #[cfg(test)]
    pub fn is_duplicate(&self, word: &str) -> bool {
        self.existing_words.contains(word)
    }

    /// Adds a vocabulary card to the generator
    ///
    /// # Arguments
    ///
    /// * `vocab_card` - The vocabulary card to add
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the card was added (true)
    /// or was a duplicate (false)
    pub fn add_card(&mut self, vocab_card: VocabularyCard) -> Result<bool> {
        // Check for duplicates before moving the card
        if self.existing_words.contains(&vocab_card.word) {
            self.duplicate_count += 1;
            return Ok(false); // Duplicate
        }

        // Clone the word before moving the card
        let word = vocab_card.word.clone();

        // Add the card and track the word
        self.cards.push(vocab_card);
        self.existing_words.insert(word);
        Ok(true)
    }

    /// Writes the cards to a JSON file
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the JSON file should be written
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure of the write operation
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // Convert cards to JSON-serializable format
        #[derive(serde::Serialize)]
        struct JsonCard {
            word: String,
            translation: String,
            example: Option<String>,
            learning_status: String,
        }

        let json_cards: Vec<JsonCard> = self.cards
            .iter()
            .map(|card| JsonCard {
                word: card.word.clone(),
                translation: card.translation.clone(),
                example: card.example.clone(),
                learning_status: match card.status {
                    LearningStatus::New => "new".to_string(),
                    LearningStatus::Learning => "learning".to_string(),
                    LearningStatus::Known => "known".to_string(),
                },
            })
            .collect();

        // Write to file with pretty printing
        let file = std::fs::File::create(path.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to create JSON file: {}", e))?;
        
        serde_json::to_writer_pretty(file, &json_cards)
            .map_err(|e| anyhow::anyhow!("Failed to write JSON: {}", e))?;

        Ok(())
    }

    /// Returns the current statistics about the generator
    pub fn stats(&self) -> JsonGeneratorStats {
        JsonGeneratorStats {
            total_cards: self.cards.len(),
            duplicates: self.duplicate_count,
            elapsed: self.start_time.elapsed(),
        }
    }
}

impl JsonGeneratorTrait for JsonGenerator {
    fn add_card(&mut self, card: VocabularyCard) -> Result<bool> {
        self.add_card(card)
    }

    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.write_to_file(path)
    }
}

/// Statistics about the JSON generation process
#[derive(Debug, Clone)]
pub struct JsonGeneratorStats {
    /// Total number of cards processed
    pub total_cards: usize,
    /// Number of duplicate cards skipped
    pub duplicates: usize,
    /// Time taken for generation
    pub elapsed: std::time::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::LearningStatus;
    use serde_json::{Value, from_str};
    use tempfile::NamedTempFile;
    use std::fs;

    fn create_test_card(
        word: &str,
        translation: &str,
        example: Option<&str>,
        status: LearningStatus,
    ) -> VocabularyCard {
        VocabularyCard {
            word: word.to_string(),
            translation: translation.to_string(),
            example: example.map(|s| s.to_string()),
            status,
        }
    }

    #[test]
    fn test_new_generator() {
        let generator = JsonGenerator::new();
        assert!(generator.cards.is_empty());
        assert!(generator.existing_words.is_empty());
    }

    #[test]
    fn test_add_card() {
        let mut generator = JsonGenerator::new();

        // Add first card
        let card1 = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
        assert!(generator.add_card(card1).unwrap());
        assert_eq!(generator.cards.len(), 1);
        assert!(generator.is_duplicate("hello"));

        // Add duplicate card
        let card2 = create_test_card("hello", "hola", Some("Hello again!"), LearningStatus::Learning);
        assert!(!generator.add_card(card2).unwrap());
        assert_eq!(generator.cards.len(), 1); // Still only one card

        // Add different card
        let card3 = create_test_card("goodbye", "adiós", Some("Goodbye, world!"), LearningStatus::Known);
        assert!(generator.add_card(card3).unwrap());
        assert_eq!(generator.cards.len(), 2);
    }

    #[test]
    fn test_write_to_file() {
        let mut generator = JsonGenerator::new();

        // Add some cards
        let card1 = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
        generator.add_card(card1).unwrap();

        let card2 = create_test_card("goodbye", "adiós", Some("Goodbye, world!"), LearningStatus::Known);
        generator.add_card(card2).unwrap();

        // Write to temporary file
        let temp_file = NamedTempFile::new().unwrap();
        generator.write_to_file(&temp_file).unwrap();

        // Verify file exists and has content
        let metadata = fs::metadata(&temp_file).unwrap();
        assert!(metadata.len() > 0);

        // Verify JSON content
        let content = fs::read_to_string(&temp_file).unwrap();
        let json: Value = from_str(&content).unwrap();
        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 2);

        // Verify card content
        let cards = json.as_array().unwrap();
        let first_card = &cards[0];
        assert_eq!(first_card["word"], "hello");
        assert_eq!(first_card["translation"], "hola");
        assert_eq!(first_card["example"], "Hello, world!");
        assert_eq!(first_card["learning_status"], "new");

        let second_card = &cards[1];
        assert_eq!(second_card["word"], "goodbye");
        assert_eq!(second_card["translation"], "adiós");
        assert_eq!(second_card["example"], "Goodbye, world!");
        assert_eq!(second_card["learning_status"], "known");
    }

    #[test]
    fn test_write_to_file_invalid_path() {
        let generator = JsonGenerator::new();
        let result = generator.write_to_file("/invalid/path/with/nulls/\0");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_generator() {
        let generator = JsonGenerator::new();
        let temp_file = NamedTempFile::new().unwrap();

        // Should still be able to write an empty generator
        generator.write_to_file(&temp_file).unwrap();

        // Verify file exists and contains empty array
        let content = fs::read_to_string(&temp_file).unwrap();
        let json: Value = from_str(&content).unwrap();
        assert!(json.is_array());
        assert!(json.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_stats() {
        let mut generator = JsonGenerator::new();
        
        // Add some cards
        let card1 = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
        generator.add_card(card1).unwrap();
        
        let card2 = create_test_card("hello", "hola", Some("Hello again!"), LearningStatus::Learning);
        generator.add_card(card2).unwrap(); // Duplicate
        
        let card3 = create_test_card("goodbye", "adiós", Some("Goodbye, world!"), LearningStatus::Known);
        generator.add_card(card3).unwrap();

        let stats = generator.stats();
        assert_eq!(stats.total_cards, 2); // Only unique cards
        assert_eq!(stats.duplicates, 1); // One duplicate
        assert!(stats.elapsed > std::time::Duration::from_secs(0));
    }
} 