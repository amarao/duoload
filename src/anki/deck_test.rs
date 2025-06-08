#[cfg(test)]
mod tests {
    use crate::anki::AnkiPackageBuilder;
    use crate::duocards::models::{LearningStatus, VocabularyCard};
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
        let builder = AnkiPackageBuilder::new("Test Deck");
        // Verify a new builder has no duplicates
        assert!(!builder.is_duplicate("test"));
    }

    #[test]
    fn test_add_note() {
        let mut builder = AnkiPackageBuilder::new("Test Deck");

        // Add first note
        let card1 = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
        assert!(builder.add_note(card1).unwrap());
        assert!(builder.is_duplicate("hello"));

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
        assert!(builder.is_duplicate("goodbye"));
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
