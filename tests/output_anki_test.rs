use duoload::duocards::models::{LearningStatus, VocabularyCard};
use duoload::output::anki::AnkiPackageBuilder;
use duoload::output::{OutputBuilder, OutputDestination};
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
    let mut builder = AnkiPackageBuilder::new("Test Deck");
    let card = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
    assert!(builder.add_note(card).unwrap());
}

#[test]
fn test_add_note() {
    let mut builder = AnkiPackageBuilder::new("Test Deck");

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
    let result = builder.write(OutputDestination::File(temp_file.path()));
    assert!(result.is_ok());
}

#[test]
fn test_write_to_buffer() {
    let mut builder = AnkiPackageBuilder::new("Test Deck");
    let card = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
    builder.add_note(card).unwrap();

    let mut buffer = Vec::new();
    let result = builder.write(OutputDestination::Writer(&mut buffer));
    assert!(result.is_err()); // Anki output only supports file output
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Anki output is only supported for file output")
    );
}

#[test]
fn test_empty_deck() {
    let builder = AnkiPackageBuilder::new("Empty Deck");
    let temp_file = NamedTempFile::new().unwrap();
    let result = builder.write(OutputDestination::File(temp_file.path()));
    assert!(result.is_ok()); // Should be able to write an empty deck
}
