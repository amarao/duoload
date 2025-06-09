use crate::anki::note::{VocabularyNote, create_vocabulary_model};
use crate::duocards::models::VocabularyCard;
use crate::error::{DuoloadError, Result};
use crate::output::{OutputBuilder, OutputDestination};
use genanki_rs::Deck;
use std::collections::HashSet;

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
            "Vocabulary imported from Duocards",
        );

        Self {
            deck,
            model,
            existing_words: HashSet::new(),
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

    fn write(&self, dest: OutputDestination<'_>) -> Result<()> {
        match dest {
            OutputDestination::Writer(_) => {
                // Anki packages can only be written to files
                Err(DuoloadError::AnkiOutputNotSupported)
            }
            OutputDestination::File(path) => {
                // Convert path to string and write the Anki package
                let path_str = path
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;
                self.deck
                    .write_to_file(path_str)
                    .map_err(|e| anyhow::anyhow!("Failed to write Anki package: {}", e))?;
                Ok(())
            }
        }
    }
}
