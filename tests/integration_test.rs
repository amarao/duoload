use duoload::anki::AnkiPackageBuilder;
use duoload::duocards::models::{LearningStatus, VocabularyCard};
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

#[tokio::test]
async fn test_end_to_end_package_creation() {
    // Create a temporary file for the package
    let temp_file = NamedTempFile::new().unwrap();
    let package_path = temp_file.path();

    // Create a new deck
    let mut builder = AnkiPackageBuilder::new("Integration Test Deck");

    // Add some test cards
    let cards = vec![
        create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New),
        create_test_card(
            "goodbye",
            "adiós",
            Some("Goodbye, world!"),
            LearningStatus::Learning,
        ),
        create_test_card(
            "thank you",
            "gracias",
            Some("Thank you very much!"),
            LearningStatus::Known,
        ),
    ];

    // Add cards to deck
    for card in cards {
        assert!(builder.add_note(card).unwrap());
    }

    // Write the package
    builder.write_to_file(package_path).unwrap();

    // Verify the package file exists and has content
    let metadata = fs::metadata(package_path).unwrap();
    assert!(metadata.len() > 0);

    // Verify the package file is a valid ZIP archive (Anki packages are ZIP files)
    let file_content = fs::read(package_path).unwrap();
    assert!(file_content.starts_with(b"PK\x03\x04")); // ZIP file signature
}

#[tokio::test]
async fn test_duplicate_handling() {
    let mut builder = AnkiPackageBuilder::new("Duplicate Test Deck");

    // Add initial card
    let card1 = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
    assert!(builder.add_note(card1).unwrap());

    // Try to add the same word with different content
    let card2 = create_test_card(
        "hello",                   // Same word
        "bonjour",                 // Different translation
        Some("Bonjour le monde!"), // Different example
        LearningStatus::Learning,  // Different status
    );
    assert!(!builder.add_note(card2).unwrap()); // Should be rejected as duplicate

    // Add a different word
    let card3 = create_test_card(
        "goodbye",
        "adiós",
        Some("Goodbye, world!"),
        LearningStatus::Known,
    );
    assert!(builder.add_note(card3).unwrap());

    // Verify we can write the package
    let temp_file = NamedTempFile::new().unwrap();
    builder.write_to_file(&temp_file).unwrap();

    // Verify file exists and has content
    let metadata = fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0);

    // Verify it's a valid ZIP archive
    let file_content = fs::read(&temp_file).unwrap();
    assert!(file_content.starts_with(b"PK\x03\x04")); // ZIP file signature
}

#[tokio::test]
async fn test_empty_deck_creation() {
    let builder = AnkiPackageBuilder::new("Empty Deck");
    let temp_file = NamedTempFile::new().unwrap();

    // Should be able to write an empty deck
    builder.write_to_file(&temp_file).unwrap();

    // Verify file exists and has content
    let metadata = fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0);

    // Verify it's a valid ZIP archive
    let file_content = fs::read(&temp_file).unwrap();
    assert!(file_content.starts_with(b"PK\x03\x04")); // ZIP file signature
}

#[tokio::test]
async fn test_large_deck_creation() {
    let mut builder = AnkiPackageBuilder::new("Large Test Deck");

    // Add 100 different cards
    for i in 0..100 {
        let word = format!("word{}", i);
        let translation = format!("translation{}", i);
        let example = format!("Example sentence for word{}", i);
        let status = match i % 3 {
            0 => LearningStatus::New,
            1 => LearningStatus::Learning,
            _ => LearningStatus::Known,
        };

        let card = create_test_card(&word, &translation, Some(&example), status);
        assert!(builder.add_note(card).unwrap());
    }

    // Write the package
    let temp_file = NamedTempFile::new().unwrap();
    builder.write_to_file(&temp_file).unwrap();

    // Verify file exists and has content
    let metadata = fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0);

    // Verify it's a valid ZIP archive
    let file_content = fs::read(&temp_file).unwrap();
    assert!(file_content.starts_with(b"PK\x03\x04")); // ZIP file signature
}
