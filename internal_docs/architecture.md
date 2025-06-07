# Duoload Architecture

## Core Flow

The application follows a simple, linear flow to transfer vocabulary from Duocards to Anki:

1. **User Input**
   ```
   duoload --deck-id "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=" --output-file "my_deck.apkg"
   ```

2. **Data Flow**
   ```
   Duocards API (public) → Transfer Processor → Anki Package
   ```

## Components

### 1. CLI Interface (`src/main.rs`)
- Parses deck ID and output file path
- Provides progress feedback
- Example output:
  ```
  Initializing new Anki file at 'my_deck.apkg'...
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
- Coordinates between Duocards client and Anki generator
- Processes cards in a streaming fashion

### 4. Anki Generator (`src/anki/`)
- Creates Anki package using genanki-rs
- Maps Duocards data to Anki format:
  - Word → Front
  - Translation → Back
  - Example → Example field
  - Status → Tag (duoload_new, duoload_learning, duoload_known)
- Writes final .apkg file

## Happy Path Sequence

1. User runs command with valid deck ID and output path
2. CLI validates inputs and creates client
3. Client validates deck ID and fetches vocabulary pages from Duocards
4. Transfer processor:
   - Receives cards from client
   - Checks for duplicates
   - Streams processed cards to Anki generator
5. Anki generator creates package and saves file
6. User gets success message with card count
