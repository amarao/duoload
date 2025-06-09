use crate::duocards::models::VocabularyCard;
use crate::error::Result;
use crate::output::{OutputBuilder, OutputDestination};
use serde_json;
use std::collections::HashSet;
use std::io::Write;
use std::time::Instant;

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

    fn write(&self, dest: OutputDestination<'_>) -> Result<()> {
        match dest {
            OutputDestination::Writer(writer) => {
                // Write directly to the writer
                serde_json::to_writer_pretty(writer, &self.cards)
                    .map_err(|e| anyhow::anyhow!("Failed to write JSON: {}", e))?;
            }
            OutputDestination::File(path) => {
                // Create a file and write to it
                let file = std::fs::File::create(path)?;
                let mut writer = std::io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &self.cards)
                    .map_err(|e| anyhow::anyhow!("Failed to write JSON: {}", e))?;
                writer.flush()?;
            }
        }

        println!(
            "JSON written successfully at {:?}",
            self.start_time.elapsed()
        );

        Ok(())
    }
}
