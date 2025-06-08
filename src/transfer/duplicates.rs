use std::collections::HashSet;

pub struct DuplicateHandler {
    processed_words: HashSet<String>,
}

impl DuplicateHandler {
    pub fn new() -> Self {
        Self {
            processed_words: HashSet::new(),
        }
    }

    pub fn is_duplicate(&mut self, word: &str) -> bool {
        !self.processed_words.insert(word.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_handler_new() {
        let handler = DuplicateHandler::new();
        assert!(handler.processed_words.is_empty());
    }

    #[test]
    fn test_duplicate_handler_basic() {
        let mut handler = DuplicateHandler::new();

        // First time seeing a word
        assert!(!handler.is_duplicate("hello"));
        assert!(handler.processed_words.contains("hello"));

        // Second time seeing the same word
        assert!(handler.is_duplicate("hello"));

        // Different word
        assert!(!handler.is_duplicate("world"));
        assert!(handler.processed_words.contains("world"));
    }

    #[test]
    fn test_duplicate_handler_case_sensitive() {
        let mut handler = DuplicateHandler::new();

        assert!(!handler.is_duplicate("Hello"));
        assert!(!handler.is_duplicate("hello")); // Different due to case
        assert!(handler.is_duplicate("Hello")); // Duplicate
    }
}
