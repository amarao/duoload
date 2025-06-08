# Duoload Architecture

## Core Flow

The application follows a simple, linear flow to transfer vocabulary from Duocards to either Anki or JSON format:

1. **User Input**
   ```
   # For Anki output (file only)
   duoload --deck-id "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=" --anki-file "my_deck.apkg"
   
   # For JSON file output
   duoload --deck-id "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=" --json-file "my_deck.json"

   # For JSON stdout output (JSON only)
   duoload --deck-id "RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=" --json | jq '.[] | select(.learning_status == "new")'
   ```

2. **Data Flow**
   ```
   Duocards API (public) → Transfer Processor → (Anki Generator → File | JSON Generator → (File | Stdout))
   ```

## Components

### 1. CLI Interface (`src/main.rs`)
- Parses deck ID and output format options
- Validates mutually exclusive output options (--anki-file, --json-file, or --json)
- Provides progress feedback
- Example outputs:
  ```
  # Anki output
  Initializing export to 'my_deck.apkg'...
  Processing page 1... done.
  Processing page 2... done.
  Export complete. 1250 cards saved to my_deck.apkg.

  # JSON output (both file and stdout)
  Processing page 1... done.
  Processing page 2... done.
  Export complete. 1250 cards processed.
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
- Manages output destination selection:
  - For JSON: Handles writing to either file or stdout based on output_path
  - For Anki: Always writes to specified file path
- The processor creates the appropriate `Writer` (file or stdout) and passes it to the builder's `write<W: Write>(&self, writer: &mut W)` method.
- Builders do not assume the destination is a file.
- For Anki, the builder returns an error if the writer is not a file.
- For JSON, the builder supports any writer.
- Ensures progress messages go to stderr when using stdout output

### 4. Anki Generator (`src/anki/`)
- Creates Anki package using genanki-rs
- Maps Duocards data to Anki format:
  - Word → Front
  - Translation → Back
  - Example → Example field
  - Status → Tag (duoload_new, duoload_learning, duoload_known)
- Implements `write<W: Write>(&self, writer: &mut W)` for output.
- Writes final .apkg file

### 5. JSON Generator (`src/output/json.rs`)
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
- Focuses on JSON generation and duplicate handling
- Implements `write<W: Write>(&self, writer: &mut W)` for output.
- Supports any writer (file, stdout, buffer, etc).
- Generates pretty-printed JSON for readability

## Happy Path Sequence

1. User runs command with valid deck ID and output format
2. CLI validates inputs and creates appropriate generator
3. Client validates deck ID and fetches vocabulary pages from Duocards
4. Transfer processor:
   - Receives cards from client
   - Checks for duplicates
   - Streams processed cards to selected generator
   - For JSON: Determines output destination (file or stdout)
   - For Anki: Always uses specified file path
   - Processor creates the appropriate Writer and passes it to the builder's `write` method
5. Generator creates output in requested format:
   - Anki: Always writes to specified Writer
   - JSON: Writes to either file or stdout based on processor configuration
6. User gets success message with card count
   - For file output: Includes file path in message
   - For stdout: Progress messages go to stderr to avoid corrupting JSON output

## Output Handling

### File Output
- Anki packages: Always write to specified file path (required)
- JSON: Can write to specified file path
- Progress messages and final statistics go to stdout
- All output generators must implement a universal `write<W: Write>(&self, writer: &mut W)` method.
- This enables flexible output redirection (file, stdout, buffer) and easier testing.

### Stdout Output (JSON only)
- Only available for JSON output format
- JSON data written to stdout for piping to other tools
- Progress messages and statistics written to stderr
- Example usage with pipe:
  ```bash
  # Filter new cards
  duoload --deck-id "..." --json | jq '.[] | select(.learning_status == "new")'
  
  # Count cards by status
  duoload --deck-id "..." --json | jq 'group_by(.learning_status) | map({status: .[0].learning_status, count: length})'
  ```
