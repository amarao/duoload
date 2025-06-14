use crate::duocards::DuocardsClientTrait;
use crate::error::Result;
use crate::output::{OutputBuilder, OutputDestination};
use crate::transfer::DuplicateHandler;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug, Default, PartialEq)]
pub struct TransferStats {
    pub total_cards: usize,
    pub duplicates: usize,
}

pub struct TransferProcessor<C>
where
    C: DuocardsClientTrait,
{
    client: C,
    deck_id: String,
}

pub struct TransferProcessorWithBuilder<C, B>
where
    C: DuocardsClientTrait,
    B: OutputBuilder,
{
    client: C,
    builder: B,
    duplicates: DuplicateHandler,
    stats: TransferStats,
    deck_id: String,
    start_time: Instant,
    output_path: PathBuf,
}

impl<C> TransferProcessor<C>
where
    C: DuocardsClientTrait,
{
    pub fn new(client: C, deck_id: String) -> Self {
        Self { client, deck_id }
    }

    pub fn output<B: OutputBuilder, P: AsRef<Path>>(
        self,
        builder: B,
        path: P,
    ) -> TransferProcessorWithBuilder<C, B> {
        TransferProcessorWithBuilder {
            client: self.client,
            builder,
            duplicates: DuplicateHandler::new(),
            stats: TransferStats::default(),
            deck_id: self.deck_id,
            start_time: Instant::now(),
            output_path: path.as_ref().to_path_buf(),
        }
    }
}

impl<C, B> TransferProcessorWithBuilder<C, B>
where
    C: DuocardsClientTrait,
    B: OutputBuilder,
{
    pub async fn process(&mut self) -> Result<()> {
        let mut cursor = None;
        let mut page_count = 0;
        let mut total_processed = 0;

        // Print initial message with page limit info if set
        if let Some(limit) = self.client.page_limit() {
            eprintln!("Starting export (limited to {} pages)...", limit);
        } else {
            eprintln!("Starting export...");
        }

        loop {
            page_count += 1;

            // Check if we should continue based on page limit
            if !self.client.should_continue(page_count) {
                eprintln!("Page limit reached ({} pages)", page_count - 1);
                break;
            }

            eprintln!("Fetching page {}...", page_count);

            // Add a delay between page fetches (1 second)
            if page_count > 1 {
                sleep(Duration::from_secs(1)).await;
            }

            // Fetch a page of cards
            let response = self.client.fetch_page(&self.deck_id, cursor).await?;
            let cards = self.client.convert_to_vocabulary_cards(&response);
            let cards_len = cards.len();
            eprintln!("Page {} fetched with {} cards", page_count, cards_len);

            // Process each card
            for card in cards.into_iter() {
                if self.duplicates.try_remember(&card.word) {
                    self.stats.duplicates += 1;
                    continue;
                }

                if self.builder.add_note(card)? {
                    self.stats.total_cards += 1;
                }

                total_processed += 1;
                if total_processed % 100 == 0 {
                    eprintln!(
                        "Processed {} cards so far ({} added, {} duplicates) at {:?}",
                        total_processed,
                        self.stats.total_cards,
                        self.stats.duplicates,
                        self.start_time.elapsed()
                    );
                }
            }

            // Check if there are more pages
            if !response.data.node.cards.page_info.has_next_page {
                eprintln!("No more pages to process");
                break;
            }

            cursor = response.data.node.cards.page_info.end_cursor;
        }

        // Print completion message with appropriate context
        if let Some(limit) = self.client.page_limit() {
            eprintln!(
                "Page limit reached ({} pages). Total cards: {}, Duplicates: {} in {:?}",
                limit,
                self.stats.total_cards,
                self.stats.duplicates,
                self.start_time.elapsed()
            );
        } else {
            eprintln!(
                "All pages processed. Total cards: {}, Duplicates: {} in {:?}",
                self.stats.total_cards,
                self.stats.duplicates,
                self.start_time.elapsed()
            );
        }

        // Write the processed data to output
        self.write_output()?;

        // Print final statistics to stderr
        self.print_stats();

        Ok(())
    }

    #[cfg(test)] // Used in tests
    pub fn stats(&self) -> &TransferStats {
        &self.stats
    }

    pub fn print_stats(&self) {
        eprintln!("Export completed successfully!");
        eprintln!("Total cards saved: {}", self.stats.total_cards);
        eprintln!("Duplicates skipped: {}", self.stats.duplicates);
        eprintln!("Total execution time: {:?}", self.start_time.elapsed());
    }

    pub fn write_output(&self) -> Result<()> {
        eprintln!("Writing deck to output...");

        let result = if self.output_path.as_os_str() == "-" {
            // Write to stdout, ensure progress messages go to stderr
            let stdout = io::stdout();
            let mut writer = stdout.lock();
            self.builder.write(OutputDestination::Writer(&mut writer))
        } else {
            // Write to file
            self.builder
                .write(OutputDestination::File(&self.output_path))
        };

        match result {
            Ok(_) => {
                eprintln!("Deck written successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error writing deck: {}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::{
        Card, CardConnection, CardEdge, Deck, DuocardsResponse, Extensions, LearningStatus,
        PageInfo, ResponseData, VocabularyCard,
    };
    use crate::output::OutputBuilder;
    use std::io::{Cursor, Write};
    use std::sync::Arc;
    use std::sync::Mutex;

    // Test-specific implementations
    #[derive(Clone)]
    struct TestDuocardsClient {
        responses: Arc<Mutex<Vec<DuocardsResponse>>>,
        page_limit: Option<u32>,
    }

    impl TestDuocardsClient {
        fn new(responses: Vec<DuocardsResponse>) -> Self {
            Self {
                responses: Arc::new(Mutex::new(responses)),
                page_limit: None,
            }
        }

        fn with_page_limit(mut self, limit: u32) -> Self {
            self.page_limit = Some(limit);
            self
        }
    }

    #[async_trait::async_trait]
    impl crate::duocards::DuocardsClientTrait for TestDuocardsClient {
        async fn fetch_page(
            &self,
            _deck_id: &str,
            _cursor: Option<String>,
        ) -> Result<DuocardsResponse> {
            let mut responses = self.responses.lock().unwrap();
            if responses.is_empty() {
                panic!("No more test responses available");
            }
            Ok(responses.remove(0))
        }

        fn convert_to_vocabulary_cards(&self, response: &DuocardsResponse) -> Vec<VocabularyCard> {
            response
                .data
                .node
                .cards
                .edges
                .iter()
                .map(|edge| VocabularyCard {
                    word: edge.node.front.clone(),
                    translation: edge.node.back.clone(),
                    example: edge.node.hint.clone(),
                    status: if edge.node.known_count >= 5 {
                        LearningStatus::Known
                    } else if edge.node.known_count > 0 {
                        LearningStatus::Learning
                    } else {
                        LearningStatus::New
                    },
                })
                .collect()
        }

        fn should_continue(&self, current_page: u32) -> bool {
            match self.page_limit {
                Some(limit) => current_page <= limit,
                None => true,
            }
        }

        fn page_limit(&self) -> Option<u32> {
            self.page_limit
        }
    }

    #[derive(Clone)]
    struct TestOutputBuilder {
        added_cards: Arc<Mutex<Vec<VocabularyCard>>>,
    }

    impl TestOutputBuilder {
        fn new() -> Self {
            Self {
                added_cards: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_added_cards(&self) -> Vec<VocabularyCard> {
            self.added_cards.lock().unwrap().clone()
        }
    }

    impl OutputBuilder for TestOutputBuilder {
        fn add_note(&mut self, card: VocabularyCard) -> Result<bool> {
            let mut added_cards = self.added_cards.lock().unwrap();
            if added_cards.iter().any(|c| c.word == card.word) {
                Ok(false)
            } else {
                added_cards.push(card);
                Ok(true)
            }
        }

        fn write(&self, dest: OutputDestination<'_>) -> Result<()> {
            match dest {
                OutputDestination::Writer(writer) => {
                    writer.write_all(b"TEST_OUTPUT")?;
                    Ok(())
                }
                OutputDestination::File(path) => {
                    let file = std::fs::File::create(path)?;
                    let mut writer = std::io::BufWriter::new(file);
                    writer.write_all(b"TEST_OUTPUT")?;
                    writer.flush()?;
                    Ok(())
                }
            }
        }
    }

    fn create_test_response(
        cards: Vec<VocabularyCard>,
        has_next_page: bool,
        end_cursor: Option<String>,
    ) -> DuocardsResponse {
        let card_edges: Vec<CardEdge> = cards
            .into_iter()
            .map(|card| CardEdge {
                node: Card {
                    id: "test-id".to_string(),
                    front: card.word,
                    back: card.translation,
                    hint: card.example,
                    waiting: None,
                    known_count: match card.status {
                        LearningStatus::Known => 5,
                        LearningStatus::Learning => 2,
                        LearningStatus::New => 0,
                    },
                    svg: None,
                    typename: "Card".to_string(),
                },
                cursor: "0".to_string(),
            })
            .collect();

        DuocardsResponse {
            data: ResponseData {
                node: Deck {
                    __typename: "Deck".to_string(),
                    cards: CardConnection {
                        edges: card_edges,
                        page_info: PageInfo {
                            end_cursor,
                            has_next_page,
                        },
                    },
                    id: "test-deck".to_string(),
                },
            },
            extensions: Extensions {
                release_id: Some("test-release".to_string()),
            },
        }
    }

    #[tokio::test]
    async fn test_process_single_page() -> Result<()> {
        // Create test cards
        let cards = vec![
            VocabularyCard {
                word: "hello".to_string(),
                translation: "hola".to_string(),
                example: Some("Hello, world!".to_string()),
                status: LearningStatus::New,
            },
            VocabularyCard {
                word: "world".to_string(),
                translation: "mundo".to_string(),
                example: None,
                status: LearningStatus::Known,
            },
        ];

        // Create test response
        let response = create_test_response(cards.clone(), false, None);

        // Create test client and builder
        let client = TestDuocardsClient::new(vec![response]);
        let builder = TestOutputBuilder::new();

        // Create processor and process cards
        let mut processor = TransferProcessor::new(client, "test-deck".to_string())
            .output(builder, Path::new("test_output.txt"));

        processor.process().await?;
        processor.write_output()?;

        // Verify results
        let stats = processor.stats();
        assert_eq!(stats.total_cards, 2);
        assert_eq!(stats.duplicates, 0);

        // Verify cards were added
        let added_cards = processor.builder.get_added_cards();
        assert_eq!(added_cards.len(), 2);
        assert_eq!(added_cards[0].word, "hello");
        assert_eq!(added_cards[1].word, "world");

        Ok(())
    }

    #[tokio::test]
    async fn test_process_multiple_pages() -> Result<()> {
        // Create test cards for two pages
        let page1_cards = vec![VocabularyCard {
            word: "hello".to_string(),
            translation: "hola".to_string(),
            example: Some("Hello, world!".to_string()),
            status: LearningStatus::New,
        }];

        let page2_cards = vec![VocabularyCard {
            word: "world".to_string(),
            translation: "mundo".to_string(),
            example: None,
            status: LearningStatus::Known,
        }];

        // Create test responses
        let response1 =
            create_test_response(page1_cards.clone(), true, Some("cursor1".to_string()));
        let response2 = create_test_response(page2_cards.clone(), false, None);

        // Create test client and builder
        let client = TestDuocardsClient::new(vec![response1, response2]);
        let builder = TestOutputBuilder::new();

        // Create processor and process cards
        let mut processor = TransferProcessor::new(client, "test-deck".to_string())
            .output(builder, Path::new("test_output.txt"));

        processor.process().await?;
        processor.write_output()?;

        // Verify results
        let stats = processor.stats();
        assert_eq!(stats.total_cards, 2);
        assert_eq!(stats.duplicates, 0);

        // Verify cards were added in correct order
        let added_cards = processor.builder.get_added_cards();
        assert_eq!(added_cards.len(), 2);
        assert_eq!(added_cards[0].word, "hello");
        assert_eq!(added_cards[1].word, "world");

        Ok(())
    }

    #[tokio::test]
    async fn test_process_with_duplicates() -> Result<()> {
        // Create test cards with duplicates
        let cards = vec![
            VocabularyCard {
                word: "hello".to_string(),
                translation: "hola".to_string(),
                example: Some("Hello, world!".to_string()),
                status: LearningStatus::New,
            },
            VocabularyCard {
                word: "hello".to_string(), // duplicate
                translation: "hola".to_string(),
                example: Some("Hello again!".to_string()),
                status: LearningStatus::Learning,
            },
            VocabularyCard {
                word: "world".to_string(),
                translation: "mundo".to_string(),
                example: None,
                status: LearningStatus::Known,
            },
        ];

        // Create test response
        let response = create_test_response(cards.clone(), false, None);

        // Create test client and builder
        let client = TestDuocardsClient::new(vec![response]);
        let builder = TestOutputBuilder::new();

        // Create processor and process cards
        let mut processor = TransferProcessor::new(client, "test-deck".to_string())
            .output(builder, Path::new("test_output.txt"));

        processor.process().await?;
        processor.write_output()?;

        // Verify results
        let stats = processor.stats();
        assert_eq!(stats.total_cards, 2);
        assert_eq!(stats.duplicates, 1);

        // Verify cards were added correctly
        let added_cards = processor.builder.get_added_cards();
        assert_eq!(added_cards.len(), 2);
        assert_eq!(added_cards[0].word, "hello");
        assert_eq!(added_cards[1].word, "world");

        Ok(())
    }

    #[test]
    fn test_write_to_stdout() -> Result<()> {
        let builder = TestOutputBuilder::new();
        let processor =
            TransferProcessor::new(TestDuocardsClient::new(vec![]), "test-deck".to_string())
                .output(builder, Path::new("-"));

        let mut output = Vec::new();
        {
            let mut writer = Cursor::new(&mut output);
            processor
                .builder
                .write(OutputDestination::Writer(&mut writer))?;
        }
        assert_eq!(output, b"TEST_OUTPUT");
        Ok(())
    }

    #[test]
    fn test_write_to_file() -> Result<()> {
        let builder = TestOutputBuilder::new();
        let temp_file = tempfile::NamedTempFile::new()?;
        let processor =
            TransferProcessor::new(TestDuocardsClient::new(vec![]), "test-deck".to_string())
                .output(builder, temp_file.path());

        processor.write_output()?;
        let contents = std::fs::read(temp_file.path())?;
        assert_eq!(contents, b"TEST_OUTPUT");
        Ok(())
    }

    #[tokio::test]
    async fn test_process_with_page_limit() -> Result<()> {
        // Create test cards for three pages
        let page1_cards = vec![VocabularyCard {
            word: "hello".to_string(),
            translation: "hola".to_string(),
            example: Some("Hello, world!".to_string()),
            status: LearningStatus::New,
        }];

        let page2_cards = vec![VocabularyCard {
            word: "world".to_string(),
            translation: "mundo".to_string(),
            example: None,
            status: LearningStatus::Known,
        }];

        let page3_cards = vec![VocabularyCard {
            word: "goodbye".to_string(),
            translation: "adiós".to_string(),
            example: None,
            status: LearningStatus::New,
        }];

        // Create test responses
        let response1 =
            create_test_response(page1_cards.clone(), true, Some("cursor1".to_string()));
        let response2 =
            create_test_response(page2_cards.clone(), true, Some("cursor2".to_string()));
        let response3 = create_test_response(page3_cards.clone(), false, None);

        // Create test client with page limit and builder
        let client =
            TestDuocardsClient::new(vec![response1, response2, response3]).with_page_limit(2);
        let builder = TestOutputBuilder::new();

        // Create processor and process cards
        let mut processor = TransferProcessor::new(client, "test-deck".to_string())
            .output(builder, Path::new("test_output.txt"));

        processor.process().await?;
        processor.write_output()?;

        // Verify results
        let stats = processor.stats();
        assert_eq!(stats.total_cards, 2); // Only first two pages should be processed
        assert_eq!(stats.duplicates, 0);

        // Verify cards were added in correct order
        let added_cards = processor.builder.get_added_cards();
        assert_eq!(added_cards.len(), 2);
        assert_eq!(added_cards[0].word, "hello");
        assert_eq!(added_cards[1].word, "world");

        Ok(())
    }
}
