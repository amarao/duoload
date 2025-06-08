use crate::error::{DeckIdError, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use uuid::Uuid;

/// Validates a deck ID according to the format specification.
///
/// The deck ID should be a base64 encoded string that decodes to "Deck:<UUID4>".
///
/// # Arguments
///
/// * `deck_id` - The deck ID to validate
///
/// # Returns
///
/// A Result containing either () if the deck ID is valid, or a DeckIdError if it's invalid.
pub fn validate_deck_id(deck_id: &str) -> Result<()> {
    // Try to decode base64
    let decoded = BASE64
        .decode(deck_id)
        .map_err(|e| DeckIdError::InvalidBase64(e.to_string()))?;

    // Convert to string
    let decoded_str = String::from_utf8(decoded).map_err(|e| {
        DeckIdError::InvalidFormat(format!("Invalid UTF-8 after base64 decode: {}", e))
    })?;

    // Check format
    if !decoded_str.starts_with("Deck:") {
        return Err(DeckIdError::InvalidFormat("Missing 'Deck:' prefix".to_string()).into());
    }

    // Extract UUID
    let uuid_str = decoded_str.trim_start_matches("Deck:");
    let uuid = Uuid::parse_str(uuid_str).map_err(|e| DeckIdError::InvalidUuid(e.to_string()))?;

    // Verify UUID version
    if uuid.get_version() != Some(uuid::Version::Random) {
        return Err(DeckIdError::NotUuidV4(format!(
            "Expected UUID v4, got version {:?}",
            uuid.get_version()
        ))
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::DuoloadError;

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
}
