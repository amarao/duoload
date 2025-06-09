use duoload::duocards::models::{LearningStatus, VocabularyCard};
use duoload::output::json::JsonOutputBuilder;
use duoload::output::{OutputBuilder, OutputDestination};
use std::fs::{self, File};
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

#[test]
fn test_new_builder() {
    let mut builder = JsonOutputBuilder::new();
    let card = create_test_card("hello", "hola", Some("Hello, world!"), LearningStatus::New);
    assert!(builder.add_note(card).unwrap());
}

#[test]
fn test_add_note() {
    let mut builder = JsonOutputBuilder::new();

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
    let mut builder = JsonOutputBuilder::new();

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
    let file = File::create(&temp_file).unwrap();
    let mut writer = BufWriter::new(file);
    builder
        .write(OutputDestination::Writer(&mut writer))
        .unwrap();
    writer.flush().unwrap();

    // Verify file exists and has content
    let metadata = fs::metadata(&temp_file).unwrap();
    assert!(metadata.len() > 0);

    // Verify JSON content
    let content = fs::read_to_string(&temp_file).unwrap();
    let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert_eq!(cards.len(), 2);
    assert_eq!(cards[0].word, "hello");
    assert_eq!(cards[1].word, "goodbye");
}

#[test]
fn test_write_invalid_path() {
    let mut builder = JsonOutputBuilder::new();
    let card = VocabularyCard {
        word: "test".to_string(),
        translation: "prueba".to_string(),
        example: Some("This is a test".to_string()),
        status: LearningStatus::New,
    };
    builder.add_note(card).unwrap();

    // Create a writer that will fail on write
    struct FailingWriter;
    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Test write error",
            ))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = FailingWriter;
    let result = builder.write(OutputDestination::Writer(&mut writer));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Test write error"));
}

#[test]
fn test_empty_deck() {
    let builder = JsonOutputBuilder::new();
    let temp_file = NamedTempFile::new().unwrap();
    let file = File::create(&temp_file).unwrap();
    let mut writer = BufWriter::new(file);
    builder
        .write(OutputDestination::Writer(&mut writer))
        .unwrap();
    writer.flush().unwrap();

    // Verify file exists and contains empty array
    let content = fs::read_to_string(&temp_file).unwrap();
    let cards: Vec<VocabularyCard> = serde_json::from_str(&content).unwrap();
    assert!(cards.is_empty());
}
