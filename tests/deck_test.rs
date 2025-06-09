use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use duoload::duocards::deck::validate_deck_id;
use duoload::error::{DeckIdError, DuoloadError};

// Valid test deck ID (base64 encoded "Deck:46f2b9ed-abf3-4bd8-a054-68dfa4a4203e")
const TEST_DECK_ID: &str = "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=";

#[test]
fn test_validate_deck_id() {
    // Valid deck ID
    assert!(validate_deck_id(TEST_DECK_ID).is_ok());

    // Invalid base64
    let invalid_base64 = "not-base64!";
    match validate_deck_id(invalid_base64) {
        Err(DuoloadError::DeckId(DeckIdError::InvalidBase64(_))) => (),
        _ => panic!("Expected InvalidBase64 error"),
    }

    // Invalid format (no Deck: prefix)
    let invalid_format = BASE64.encode("NotADeck:123");
    match validate_deck_id(&invalid_format) {
        Err(DuoloadError::DeckId(DeckIdError::InvalidFormat(_))) => (),
        _ => panic!("Expected InvalidFormat error"),
    }

    // Invalid UUID
    let invalid_uuid = BASE64.encode("Deck:not-a-uuid");
    match validate_deck_id(&invalid_uuid) {
        Err(DuoloadError::DeckId(DeckIdError::InvalidUuid(_))) => (),
        _ => panic!("Expected InvalidUuid error"),
    }

    // Non-v4 UUID
    let non_v4_uuid = BASE64.encode("Deck:00000000-0000-1000-8000-000000000000"); // v1 UUID
    match validate_deck_id(&non_v4_uuid) {
        Err(DuoloadError::DeckId(DeckIdError::NotUuidV4(_))) => (),
        _ => panic!("Expected NotUuidV4 error"),
    }
}
