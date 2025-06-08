use crate::duocards::models::VocabularyCard;
use crate::error::Result;
use crate::output::OutputBuilder;
use serde_json;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;
use std::io::Cursor;

/// Builder for creating JSON files from vocabulary cards.
///
/// This struct manages the creation of a JSON file containing vocabulary cards, handling:
/// - Card collection and duplicate detection
/// - JSON file generation with pretty printing
pub struct JsonOutputBuilder {
    cards: Vec<VocabularyCard>,
    existing_words: HashSet<String>,
    start_time: Instant,
}

impl Default for JsonOutputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonOutputBuilder {
    /// Creates a new JSON output builder.
    ///
    /// # Returns
    ///
    /// A new JsonOutputBuilder instance.
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            existing_words: HashSet::new(),
            start_time: Instant::now(),
        }
    }
}

impl OutputBuilder for JsonOutputBuilder {
    fn add_note(&mut self, card: VocabularyCard) -> Result<bool> {
        // Check for duplicates
        if self.existing_words.contains(&card.word) {
            return Ok(false); // Duplicate
        }

        // Clone the word before moving the card
        let word = card.word.clone();

        // Add the card
        self.cards.push(card);
        self.existing_words.insert(word);
        Ok(true)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        // Convert cards to JSON with pretty printing and write directly to the writer
        serde_json::to_writer_pretty(writer, &self.cards)
            .map_err(|e| anyhow::anyhow!("Failed to write JSON: {}", e))?;

        println!(
            "JSON written successfully at {:?}",
            self.start_time.elapsed()
        );

        Ok(())
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
        let mut builder = JsonOutputBuilder::new();
        assert!(builder.existing_words.is_empty());

        // Verify we can add a note to a new builder
        let card = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
        assert!(builder.add_note(card).unwrap());
    }

    #[test]
    fn test_add_note() {
        let mut builder = JsonOutputBuilder::new();

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
        let mut builder = JsonOutputBuilder::new();

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
        let file = File::create(&temp_file).unwrap();
        let mut writer = BufWriter::new(file);
        builder.write(&mut writer).unwrap();
        writer.flush().unwrap();

        // Verify file exists and has content
        let metadata = fs::metadata(&temp_file).unwrap();
        assert!(metadata.len() > 0);

        // Verify JSON content
        let content = fs::read_to_string(&temp_file).unwrap();
        let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].word, "hello");
        assert_eq!(cards[1].word, "goodbye");
    }

    #[test]
    fn test_write_invalid_path() {
        let mut builder = JsonOutputBuilder::new();
        let card = VocabularyCard {
            word: "test".to_string(),
            translation: "prueba".to_string(),
            example: Some("This is a test".to_string()),
            status: LearningStatus::New,
        };
        builder.add_note(card).unwrap();

        // Create a writer that will fail on write
        struct FailingWriter;
        impl Write for FailingWriter {
            fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Test write error"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mut writer = FailingWriter;
        let result = builder.write(&mut writer);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Test write error"));
    }

    #[test]
    fn test_empty_deck() {
        let builder = JsonOutputBuilder::new();
        let temp_file = NamedTempFile::new().unwrap();
        let file = File::create(&temp_file).unwrap();
        let mut writer = BufWriter::new(file);

        // Should still be able to write an empty deck
        builder.write(&mut writer).unwrap();
        writer.flush().unwrap();

        // Verify file exists and contains empty array
        let content = fs::read_to_string(&temp_file).unwrap();
        let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
        assert!(cards.is_empty());
    }
}
