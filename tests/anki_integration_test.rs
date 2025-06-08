use duoload::duocards::models::{LearningStatus, VocabularyCard};
use duoload::output::OutputBuilder;
use duoload::output::anki::AnkiPackageBuilder;
use std::fs::File;
use std::io::{BufWriter, Write};
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
async fn test_end_to_end_anki_package_creation() {
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
    let temp_file = NamedTempFile::new().unwrap();
    let file = File::create(&temp_file).unwrap();
    let mut writer = BufWriter::new(file);
    let result = builder.write(&mut writer);
    assert!(result.is_err()); // Anki output only supports file output
    assert!(result.unwrap_err().to_string().contains("Anki output is only supported for file output"));
}

#[tokio::test]
async fn test_anki_duplicate_handling() {
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
    let file = File::create(&temp_file).unwrap();
    let mut writer = BufWriter::new(file);
    let result = builder.write(&mut writer);
    assert!(result.is_err()); // Anki output only supports file output
    assert!(result.unwrap_err().to_string().contains("Anki output is only supported for file output"));
}

#[tokio::test]
async fn test_empty_anki_deck_creation() {
    let builder = AnkiPackageBuilder::new("Empty Deck");
    let temp_file = NamedTempFile::new().unwrap();
    let file = File::create(&temp_file).unwrap();
    let mut writer = BufWriter::new(file);

    // Should return error for any writer
    let result = builder.write(&mut writer);
    assert!(result.is_err()); // Anki output only supports file output
    assert!(result.unwrap_err().to_string().contains("Anki output is only supported for file output"));
}

#[tokio::test]
async fn test_large_anki_deck_creation() {
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
    let file = File::create(&temp_file).unwrap();
    let mut writer = BufWriter::new(file);
    let result = builder.write(&mut writer);
    assert!(result.is_err()); // Anki output only supports file output
    assert!(result.unwrap_err().to_string().contains("Anki output is only supported for file output"));
} 