use duoload::duocards::models::{LearningStatus, VocabularyCard};
use duoload::output::json::JsonOutputBuilder;
use duoload::output::{OutputBuilder, OutputDestination};
use serde_json;
use std::fs::File;
use std::io::BufWriter;
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
async fn test_end_to_end_json_creation() {
    // Create a new builder
    let mut builder = JsonOutputBuilder::new();

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

    // Add cards to builder
    for card in cards {
        assert!(builder.add_note(card).unwrap());
    }

    // Write to temporary file
    let temp_file = NamedTempFile::new().unwrap();
    {
        let file = File::create(&temp_file).unwrap();
        let mut writer = BufWriter::new(file);
        builder
            .write(OutputDestination::Writer(&mut writer))
            .unwrap();
        // Ensure writer is dropped (and flushed) before checking file contents
    }

    // Verify file exists and has content
    let metadata = std::fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0, "File should not be empty");

    // Verify JSON content
    let content = std::fs::read_to_string(&temp_file).unwrap();
    let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert_eq!(cards.len(), 3);
    assert_eq!(cards[0].word, "hello");
    assert_eq!(cards[1].word, "goodbye");
    assert_eq!(cards[2].word, "thank you");
}

#[tokio::test]
async fn test_json_duplicate_handling() {
    let mut builder = JsonOutputBuilder::new();

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

    // Write to temporary file
    let temp_file = NamedTempFile::new().unwrap();
    {
        let file = File::create(&temp_file).unwrap();
        let mut writer = BufWriter::new(file);
        builder
            .write(OutputDestination::Writer(&mut writer))
            .unwrap();
        // Ensure writer is dropped (and flushed) before checking file contents
    }

    // Verify file exists and has content
    let metadata = std::fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0, "File should not be empty");

    // Verify JSON content
    let content = std::fs::read_to_string(&temp_file).unwrap();
    let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert_eq!(cards.len(), 2);
    assert_eq!(cards[0].word, "hello");
    assert_eq!(cards[1].word, "goodbye");
}

#[tokio::test]
async fn test_empty_json_deck_creation() {
    let builder = JsonOutputBuilder::new();

    // Write to temporary file
    let temp_file = NamedTempFile::new().unwrap();
    {
        let file = File::create(&temp_file).unwrap();
        let mut writer = BufWriter::new(file);
        builder
            .write(OutputDestination::Writer(&mut writer))
            .unwrap();
        // Ensure writer is dropped (and flushed) before checking file contents
    }

    // Verify file exists and contains empty array
    let content = std::fs::read_to_string(&temp_file).unwrap();
    let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert!(cards.is_empty());
}

#[tokio::test]
async fn test_large_json_deck_creation() {
    let mut builder = JsonOutputBuilder::new();

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

    // Write to temporary file
    let temp_file = NamedTempFile::new().unwrap();
    {
        let file = File::create(&temp_file).unwrap();
        let mut writer = BufWriter::new(file);
        builder
            .write(OutputDestination::Writer(&mut writer))
            .unwrap();
        // Ensure writer is dropped (and flushed) before checking file contents
    }

    // Verify file exists and has content
    let metadata = std::fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0, "File should not be empty");

    // Verify JSON content
    let content = std::fs::read_to_string(&temp_file).unwrap();
    let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert_eq!(cards.len(), 100);
    for i in 0..100 {
        assert_eq!(cards[i].word, format!("word{}", i));
        assert_eq!(cards[i].translation, format!("translation{}", i));
    }
}
