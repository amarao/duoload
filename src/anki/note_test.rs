#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::{VocabularyCard, LearningStatus};
    use crate::anki::{VocabularyNote, note::create_vocabulary_model};
    use genanki_rs::Note;
    use anyhow::Result;

    fn create_test_card(word: &str, translation: &str, example: Option<&str>, status: LearningStatus) -> VocabularyCard {
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
        let card = create_test_card(
            "hello",
            "hola",
            None,
            LearningStatus::New,
        );
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
        let card = create_test_card(
            "hello",
            "hola",
            None,
            LearningStatus::New,
        );
        let note = VocabularyNote::from(card);
        let anki_note = note.to_anki_note(&model).unwrap();
        deck.add_note(anki_note);
    }
} 