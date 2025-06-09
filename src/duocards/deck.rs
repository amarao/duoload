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
