//! Vocabulary note module for converting vocabulary cards to Anki notes.
//!
//! This module provides functionality to convert vocabulary cards to Anki notes,
//! handling the mapping between our vocabulary model and Anki's note format.

use crate::duocards::models::VocabularyCard;
use anyhow::Result;
use genanki_rs::{Field, Model, Note, Template};

/// A note representing a vocabulary item that can be converted to an Anki note.
#[derive(Debug)]
pub struct VocabularyNote {
    pub word: String,
    pub translation: String,
    pub example: Option<String>,
    pub tags: Vec<String>,
}

impl From<VocabularyCard> for VocabularyNote {
    fn from(card: VocabularyCard) -> Self {
        let tags = match card.status {
            crate::duocards::models::LearningStatus::New => vec!["duoload_new".to_string()],
            crate::duocards::models::LearningStatus::Learning => {
                vec!["duoload_learning".to_string()]
            }
            crate::duocards::models::LearningStatus::Known => vec!["duoload_known".to_string()],
        };

        Self {
            word: card.word,
            translation: card.translation,
            example: card.example,
            tags,
        }
    }
}

impl VocabularyNote {
    /// Creates a new Anki note from this vocabulary note.
    ///
    /// # Arguments
    ///
    /// * `model` - The Anki model to use for the note
    ///
    /// # Returns
    ///
    /// A Result containing either the created Anki note or an error if creation fails.
    pub fn to_anki_note(&self, model: &Model) -> Result<Note> {
        let fields = vec![
            self.word.as_str(),
            self.translation.as_str(),
            self.example.as_deref().unwrap_or(""),
        ];

        let mut note = Note::new(model.clone(), fields)?;
        note = note.tags(self.tags.clone());
        Ok(note)
    }
}

/// Creates a vocabulary model for Anki notes.
///
/// This model defines the structure of vocabulary notes in Anki,
/// including fields for the word, translation, and example.
pub fn create_vocabulary_model() -> Model {
    Model::new(
        1607392319, // Model ID - fixed for consistency
        "Duoload Vocabulary",
        vec![
            Field::new("Front"),
            Field::new("Back"),
            Field::new("Example"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}\n\n{{#Example}}<div class=\"example\">{{Example}}</div>{{/Example}}"),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::LearningStatus;
    use anyhow::Result;

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
    fn test_from_vocabulary_card() {
        let card = create_test_card(
            "hello",
            "hola",
            Some("Hello, world!"),
            LearningStatus::Known,
        );
        let note = VocabularyNote::from(card);
        assert_eq!(note.word, "hello");
        assert_eq!(note.translation, "hola");
        assert_eq!(note.example, Some("Hello, world!".to_string()));
        assert_eq!(note.tags, vec!["duoload_known"]);
    }

    #[test]
    fn test_from_vocabulary_card_no_example() {
        let card = create_test_card("hello", "hola", None, LearningStatus::New);
        let note = VocabularyNote::from(card);
        assert_eq!(note.word, "hello");
        assert_eq!(note.translation, "hola");
        assert_eq!(note.example, None);
        assert_eq!(note.tags, vec!["duoload_new"]);
    }

    #[test]
    fn test_to_anki_note() -> Result<()> {
        let card = create_test_card(
            "hello",
            "hola",
            Some("Hello, world!"),
            LearningStatus::Known,
        );
        let note = VocabularyNote::from(card);
        let model = create_vocabulary_model();
        let anki_note = note.to_anki_note(&model)?;

        // We can't directly test the note's fields as they're private in genanki_rs
        // Instead, we'll verify the note was created successfully by writing it to a deck
        let mut deck = genanki_rs::Deck::new(1234, "Test Deck", "Test");
        deck.add_note(anki_note);
        Ok(())
    }

    #[test]
    fn test_create_vocabulary_model() {
        let model = create_vocabulary_model();
        // Verify the model was created with the correct ID
        assert_eq!(model.id, 1607392319);
    }

    #[test]
    fn test_note_conversion() {
        // Test with example
        let card = create_test_card(
            "hello",
            "hola",
            Some("Hello, world!"),
            LearningStatus::Known,
        );
        let note = VocabularyNote::from(card);
        let model = create_vocabulary_model();
        let anki_note = note.to_anki_note(&model).unwrap();

        // Verify the note was created by adding it to a deck
        let mut deck = genanki_rs::Deck::new(1234, "Test Deck", "Test");
        deck.add_note(anki_note);

        // Test without example
        let card = create_test_card("hello", "hola", None, LearningStatus::New);
        let note = VocabularyNote::from(card);
        let anki_note = note.to_anki_note(&model).unwrap();
        deck.add_note(anki_note);
    }
}
