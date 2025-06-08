# Duoload Architecture

## Core Flow

The application follows a simple, linear flow to transfer vocabulary from Duocards to either Anki or JSON format:

1. **User Input**
   ```
   # For Anki output
   duoload --deck-id "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=" --anki-file "my_deck.apkg"
   
   # For JSON output
   duoload --deck-id "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=" --json-file "my_deck.json"
   ```

2. **Data Flow**
   ```
   Duocards API (public) → Transfer Processor → (Anki Generator | JSON Generator)
   ```

## Components

### 1. CLI Interface (`src/main.rs`)
- Parses deck ID and output format options
- Validates mutually exclusive output options
- Provides progress feedback
- Example output:
  ```
  Initializing export to 'my_deck.apkg'...
  Processing page 1... done.
  Processing page 2... done.
  Export complete. 1250 cards saved to my_deck.apkg.
  ```

### 2. Duocards Client (`src/duocards/`)
- Fetches vocabulary cards from Duocards API
- Handles pagination automatically
- Returns structured data:
  ```rust
  struct VocabularyCard {
      word: String,
      translation: String,
      example: Option<String>,
      status: LearningStatus,
  }
  ```

### 3. Transfer Processor (`src/transfer/`)
- Orchestrates the transfer process
- Handles duplicate detection
- Coordinates between Duocards client and output generators
- Processes cards in a streaming fashion
- Format-agnostic core logic

### 4. Anki Generator (`src/anki/`)
- Creates Anki package using genanki-rs
- Maps Duocards data to Anki format:
  - Word → Front
  - Translation → Back
  - Example → Example field
  - Status → Tag (duoload_new, duoload_learning, duoload_known)
- Writes final .apkg file

### 5. JSON Generator (`src/json/`)
- Serializes vocabulary cards to JSON format
- Maintains consistent structure:
  ```json
  {
    "word": string,
    "translation": string,
    "example": string,
    "learning_status": "new" | "learning" | "known"
  }
  ```
- Writes UTF-8 encoded JSON file

## Happy Path Sequence

1. User runs command with valid deck ID and output format
2. CLI validates inputs and creates appropriate generator
3. Client validates deck ID and fetches vocabulary pages from Duocards
4. Transfer processor:
   - Receives cards from client
   - Checks for duplicates
   - Streams processed cards to selected generator
5. Generator creates file in requested format
6. User gets success message with card count
