use duoload::duocards::client::DuocardsClient;
use duoload::duocards::models::LearningStatus;
use mockito::Server;
use serde_json::json;
use tokio_test::block_on;

// Valid test deck ID (base64 encoded "Deck:46f2b9ed-abf3-4bd8-a054-68dfa4a4203e")
const TEST_DECK_ID: &str = "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=";

fn create_mock_response() -> serde_json::Value {
    json!({
        "data": {
            "node": {
                "__typename": "Deck",
                "cards": {
                    "edges": [
                        {
                            "node": {
                                "id": "test-id",
                                "front": "hello",
                                "back": "hola",
                                "hint": "Hello, world!",
                                "waiting": null,
                                "knownCount": 5,
                                "svg": null,
                                "__typename": "Card"
                            },
                            "cursor": "0"
                        }
                    ],
                    "pageInfo": {
                        "endCursor": "0",
                        "hasNextPage": true
                    }
                },
                "id": TEST_DECK_ID
            }
        },
        "extensions": {
            "releaseId": "2025-06-04T14:06:15.707Z"
        }
    })
}

#[test]
fn test_fetch_page() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/graphql")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(create_mock_response().to_string())
        .create();

    // Override the base URL to use the mock server
    let mut client = DuocardsClient::new().unwrap();
    client.base_url = server.url() + "/graphql";

    let response = block_on(client.fetch_page(TEST_DECK_ID, None)).unwrap();

    mock.assert();
    assert_eq!(response.data.node.id, TEST_DECK_ID);
    assert_eq!(response.data.node.cards.edges.len(), 1);
    assert_eq!(response.data.node.cards.edges[0].node.front, "hello");
    assert_eq!(response.data.node.cards.edges[0].node.back, "hola");
    assert_eq!(response.data.node.cards.edges[0].node.known_count, 5);
    assert_eq!(
        response.data.node.cards.page_info.end_cursor,
        Some("0".to_string())
    );
    assert!(response.data.node.cards.page_info.has_next_page);
}

#[test]
fn test_convert_to_vocabulary_cards() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/graphql")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(create_mock_response().to_string())
        .create();

    // Override the base URL to use the mock server
    let mut client = DuocardsClient::new().unwrap();
    client.base_url = server.url() + "/graphql";

    let response = block_on(client.fetch_page(TEST_DECK_ID, None)).unwrap();
    let cards = client.convert_to_vocabulary_cards(&response);

    mock.assert();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].word, "hello");
    assert_eq!(cards[0].translation, "hola");
    assert_eq!(cards[0].example, Some("Hello, world!".to_string()));
    assert!(matches!(cards[0].status, LearningStatus::Known));
}

#[test]
fn test_page_limit() {
    let client = DuocardsClient::new().unwrap();
    
    // Test without page limit
    assert!(client.should_continue(1));
    assert!(client.should_continue(100));
    
    // Test with page limit
    let client = client.with_page_limit(5);
    assert!(client.should_continue(1));
    assert!(client.should_continue(5));
    assert!(!client.should_continue(6));
    assert!(!client.should_continue(100));
}

#[test]
fn test_page_limit_with_fetch() {
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/graphql")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(create_mock_response().to_string())
        .create();

    // Create client with page limit
    let mut client = DuocardsClient::new().unwrap().with_page_limit(3);
    client.base_url = server.url() + "/graphql";

    // Test that client respects page limit
    assert!(client.should_continue(1));
    assert!(client.should_continue(2));
    assert!(client.should_continue(3));
    assert!(!client.should_continue(4));

    // Test that fetch still works with page limit
    let response = block_on(client.fetch_page(TEST_DECK_ID, None)).unwrap();
    mock.assert();
    assert_eq!(response.data.node.id, TEST_DECK_ID);
}
