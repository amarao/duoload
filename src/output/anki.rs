use crate::anki::note::{VocabularyNote, create_vocabulary_model};
use crate::duocards::models::VocabularyCard;
use crate::error::{DuoloadError, Result};
use crate::output::OutputBuilder;
use genanki_rs::Deck;
use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

/// Builder for creating Anki packages from vocabulary cards.
///
/// This struct manages the creation of an Anki package, handling:
/// - Deck creation and configuration
/// - Note addition with duplicate detection
/// - Package file generation
pub struct AnkiPackageBuilder {
    pub deck: Deck,
    pub model: genanki_rs::Model,
    existing_words: HashSet<String>,
    start_time: Instant,
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
        let start_time = Instant::now();

        let model = create_vocabulary_model();

        let deck = Deck::new(
            2059400110, // Deck ID - fixed for consistency
            deck_name,
            "Vocabulary imported from Duocards",
        );

        Self {
            deck,
            model,
            existing_words: HashSet::new(),
            start_time,
        }
    }
}

impl OutputBuilder for AnkiPackageBuilder {
    fn add_note(&mut self, vocab_card: VocabularyCard) -> Result<bool> {
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

    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let result = self
            .deck
            .write_to_file(
                path.as_ref()
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid file path: {:?}", path.as_ref()))?,
            )
            .map_err(|e| anyhow::anyhow!("Failed to write Anki package: {}", e));

        match &result {
            Ok(_) => println!(
                "Deck written successfully at {:?}",
                self.start_time.elapsed()
            ),
            Err(e) => println!(
                "Failed to write deck: {} at {:?}",
                e,
                self.start_time.elapsed()
            ),
        }

        result.map_err(DuoloadError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::LearningStatus;
    use std::fs;
    use tempfile::NamedTempFile;

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
    fn test_new_builder() {
        let mut builder = AnkiPackageBuilder::new("Test Deck");
        assert!(builder.existing_words.is_empty());

        // Verify we can add a note to a new builder
        let card = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
        assert!(builder.add_note(card).unwrap());
    }

    #[test]
    fn test_add_note() {
        let mut builder = AnkiPackageBuilder::new("Test Deck");

        // Add first note
        let card1 = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
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
        let card1 = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
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
