use duoload::duocards::models::{LearningStatus, VocabularyCard};
use duoload::output::OutputBuilder;
use duoload::output::json::JsonOutputBuilder;
use std::fs;
use tempfile::NamedTempFile;
use serde_json;

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
    // Create a temporary file for the JSON output
    let temp_file = NamedTempFile::new().unwrap();
    let json_path = temp_file.path();

    // Create a new JSON builder
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

    // Write the JSON file
    builder.write_to_file(json_path).unwrap();

    // Verify the file exists and has content
    let metadata = fs::metadata(json_path).unwrap();
    assert!(metadata.len() > 0);

    // Verify the JSON content is valid and contains our cards
    let content = fs::read_to_string(json_path).unwrap();
    let parsed_cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed_cards.len(), 3);
    assert_eq!(parsed_cards[0].word, "hello");
    assert_eq!(parsed_cards[1].word, "goodbye");
    assert_eq!(parsed_cards[2].word, "thank you");
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

    // Verify we can write the JSON file
    let temp_file = NamedTempFile::new().unwrap();
    builder.write_to_file(&temp_file).unwrap();

    // Verify file exists and has content
    let metadata = fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0);

    // Verify JSON content
    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed_cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed_cards.len(), 2);
    assert_eq!(parsed_cards[0].word, "hello");
    assert_eq!(parsed_cards[1].word, "goodbye");
}

#[tokio::test]
async fn test_empty_json_deck_creation() {
    let builder = JsonOutputBuilder::new();
    let temp_file = NamedTempFile::new().unwrap();

    // Should be able to write an empty deck
    builder.write_to_file(&temp_file).unwrap();

    // Verify file exists and has content
    let metadata = fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0);

    // Verify JSON content is an empty array
    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed_cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert!(parsed_cards.is_empty());
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

    // Write the JSON file
    let temp_file = NamedTempFile::new().unwrap();
    builder.write_to_file(&temp_file).unwrap();

    // Verify file exists and has content
    let metadata = fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0);

    // Verify JSON content
    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed_cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed_cards.len(), 100);
    
    // Verify a few random cards
    assert_eq!(parsed_cards[0].word, "word0");
    assert_eq!(parsed_cards[50].word, "word50");
    assert_eq!(parsed_cards[99].word, "word99");
} 