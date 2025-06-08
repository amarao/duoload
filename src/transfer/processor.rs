use crate::anki::AnkiPackageBuilderTrait;
use crate::duocards::DuocardsClientTrait;
use crate::error::Result;
use crate::transfer::DuplicateHandler;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Default, PartialEq)]
pub struct TransferStats {
    pub total_cards: usize,
    pub duplicates: usize,
}

pub struct TransferProcessor<C, B>
where
    C: DuocardsClientTrait,
    B: AnkiPackageBuilderTrait,
{
    client: C,
    builder: B,
    duplicates: DuplicateHandler,
    stats: TransferStats,
    deck_id: String,
}

impl<C, B> TransferProcessor<C, B>
where
    C: DuocardsClientTrait,
    B: AnkiPackageBuilderTrait,
{
    pub fn new(client: C, builder: B, deck_id: String) -> Self {
        Self {
            client,
            builder,
            duplicates: DuplicateHandler::new(),
            stats: TransferStats::default(),
            deck_id,
        }
    }

    pub async fn process_all(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let mut cursor = None;
        let mut page_count = 0;
        let mut total_processed = 0;

        loop {
            page_count += 1;
            println!("[DEBUG] Fetching page {}...", page_count);

            // Fetch a page of cards
            let response = self.client.fetch_page(&self.deck_id, cursor).await?;
            let cards = self.client.convert_to_vocabulary_cards(&response);
            let cards_len = cards.len();
            println!(
                "[DEBUG] Page {} fetched with {} cards",
                page_count, cards_len
            );

            // Process each card
            for card in cards.into_iter() {
                if self.duplicates.is_duplicate(&card.word) {
                    self.stats.duplicates += 1;
                    continue;
                }

                if self.builder.add_note(card)? {
                    self.stats.total_cards += 1;
                }

                total_processed += 1;
                if total_processed % 100 == 0 {
                    println!(
                        "[DEBUG] Processed {} cards so far ({} added, {} duplicates) at {:?}",
                        total_processed,
                        self.stats.total_cards,
                        self.stats.duplicates,
                        start_time.elapsed()
                    );
                }
            }

            // Check if there are more pages
            if !response.data.node.cards.page_info.has_next_page {
                println!("[DEBUG] No more pages to process");
                break;
            }

            cursor = response.data.node.cards.page_info.end_cursor;
        }

        println!(
            "[DEBUG] All pages processed. Total cards: {}, Duplicates: {} in {:?}",
            self.stats.total_cards,
            self.stats.duplicates,
            start_time.elapsed()
        );
        Ok(())
    }

    pub fn stats(&self) -> &TransferStats {
        &self.stats
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        println!("[DEBUG] Writing deck to file...");
        let result = self.builder.write_to_file(path);
        println!("[DEBUG] Deck written successfully");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duocards::models::{
        Card, CardConnection, CardEdge, Deck, DuocardsResponse, Extensions, LearningStatus,
        PageInfo, ResponseData, VocabularyCard,
    };
    use std::sync::Arc;
    use std::sync::Mutex;

    // Test-specific implementations
    #[derive(Clone)]
    struct TestDuocardsClient {
        responses: Arc<Mutex<Vec<DuocardsResponse>>>,
    }

    impl TestDuocardsClient {
        fn new(responses: Vec<DuocardsResponse>) -> Self {
            Self {
                responses: Arc::new(Mutex::new(responses)),
            }
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
    }

    #[derive(Clone)]
    struct TestAnkiPackageBuilder {
        added_cards: Arc<Mutex<Vec<VocabularyCard>>>,
    }

    impl TestAnkiPackageBuilder {
        fn new() -> Self {
            Self {
                added_cards: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_added_cards(&self) -> Vec<VocabularyCard> {
            self.added_cards.lock().unwrap().clone()
        }
    }

    impl crate::anki::AnkiPackageBuilderTrait for TestAnkiPackageBuilder {
        fn add_note(&mut self, card: VocabularyCard) -> Result<bool> {
            let mut added_cards = self.added_cards.lock().unwrap();
            if added_cards.iter().any(|c| c.word == card.word) {
                Ok(false)
            } else {
                added_cards.push(card);
                Ok(true)
            }
        }

        fn write_to_file<P: AsRef<std::path::Path>>(&self, _path: P) -> Result<()> {
            Ok(())
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
    async fn test_process_all_single_page() -> Result<()> {
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
        let builder = TestAnkiPackageBuilder::new();

        // Create processor and process cards
        let mut processor = TransferProcessor::new(client, builder, "test-deck".to_string());

        processor.process_all().await?;

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
    async fn test_process_all_multiple_pages() -> Result<()> {
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
        let builder = TestAnkiPackageBuilder::new();

        // Create processor and process cards
        let mut processor = TransferProcessor::new(client, builder, "test-deck".to_string());

        processor.process_all().await?;

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
    async fn test_process_all_with_duplicates() -> Result<()> {
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
        let builder = TestAnkiPackageBuilder::new();

        // Create processor and process cards
        let mut processor = TransferProcessor::new(client, builder, "test-deck".to_string());

        processor.process_all().await?;

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
}
