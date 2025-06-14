# Duoload Technical Requirements

## 1. Project Structure

### 1.1 Cargo.toml Dependencies
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "cookies"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
genanki-rs = "0.4"
```

### 1.2 Module Structure
```
src/
├── main.rs              # Entry point and CLI argument parsing
├── duocards/
│   ├── mod.rs           # Module exports
│   ├── client.rs        # HTTP client for Duocards API
│   ├── models.rs        # Data structures for API responses
│   └── auth.rs          # Cookie handling and authentication
├── anki/
│   ├── mod.rs           # Module exports
│   ├── deck.rs          # Deck creation using genanki-rs
│   └── note.rs          # Note creation and mapping
├── transfer/
│   ├── mod.rs           # Module exports
│   ├── processor.rs     # Main transfer logic
│   └── duplicates.rs    # Duplicate detection and handling
└── error.rs             # Custom error types
```

## 2. CLI Interface Requirements

### 2.1 Command Structure
Using `clap` with derive macros:

```rust
#[derive(Parser)]
#[command(name = "duoload")]
#[command(about = "Export data from Duocards to Anki or JSON")]
struct Args {
    #[arg(long, value_name = "DECK_ID", help = "Duocards deck ID")]
    deck_id: String,
    
    #[arg(long, value_name = "FILE", help = "Output Anki package file path (.apkg)")]
    anki_file: Option<PathBuf>,
    
    #[arg(long, value_name = "FILE", help = "Output JSON file path (.json)")]
    json_file: Option<PathBuf>,

    #[arg(long, help = "Output JSON to stdout")]
    json: bool,

    #[arg(long, value_name = "N", help = "Limit export to N pages (default: all pages)")]
    pages: Option<u32>,
}
```

### 2.2 Validation Requirements
- Deck ID format validation (base64 encoded Deck:UUID)
- Output file path validation (writable directory)
- Exactly one output format must be specified (either --anki-file, --json-file, or --json)
- When using --json, ensure stdout is not a terminal if progress messages are enabled
- Page limit must be a positive integer when specified

## 3. Data Models

### 3.1 Duocards API Models
```rust
#[derive(Debug, Deserialize)]
struct DuocardsResponse {
    data: Vec<VocabularyCard>,
    pagination: PaginationInfo,
}

#[derive(Debug, Deserialize)]
struct VocabularyCard {
    word: String,
    translation: String,
    example: Option<String>,
    status: LearningStatus,
}

#[derive(Debug, Deserialize)]
enum LearningStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "learning")]
    Learning,
    #[serde(rename = "known")]
    Known,
}

#[derive(Debug, Deserialize)]
struct PaginationInfo {
    current_page: u32,
    total_pages: u32,
    has_next: bool,
}
```

### 3.2 Anki Models (using genanki-rs)
```rust
use genanki_rs::{Deck, Note, Model, Field, Template};

// Create model for vocabulary cards
fn create_vocabulary_model() -> Model {
    Model::new(
        1607392319,  // Model ID
        "Duoload Vocabulary",
        vec![
            Field::new("Front"),
            Field::new("Back"), 
            Field::new("Example"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}")
                .afmt("{{Back}}<br><br>{{#Example}}Example: {{Example}}{{/Example}}"),
        ],
    )
}

struct VocabularyNote {
    word: String,
    translation: String,
    example: String,
    tags: Vec<String>,
}

impl VocabularyNote {
    fn to_anki_note(&self, model: &Model) -> Note {
        Note::new(model.clone(), vec![
            &self.word,
            &self.translation,
            &self.example,
        ])
        .tags(self.tags.clone())
    }
}
```

## 4. HTTP Client Requirements

### 4.1 Duocards API Client
```rust
struct DuocardsClient {
    client: reqwest::Client,
    deck_id: String,
    base_url: String,
    page_limit: Option<u32>,
}

impl DuocardsClient {
    async fn fetch_page(&self, page: u32) -> Result<DuocardsResponse>;
    
    fn should_continue(&self, current_page: u32) -> bool {
        match self.page_limit {
            Some(limit) => current_page <= limit,
            None => true,
        }
    }
}
```

### 4.2 Deck Validation
```rust
// In src/duocards/deck.rs
pub fn validate_deck_id(deck_id: &str) -> Result<()>;
```

### 4.3 Network Configuration
- Request timeout: 30 seconds
- Retry mechanism: Exponential backoff (1s, 2s, 4s, 8s, max 16s)
- Maximum retries: 3 attempts
- Polite delay between requests: 1-2 seconds
- User-Agent: "duoload/1.0"
- Page limit handling: Stop fetching when limit is reached or no more pages available

### 4.4 Error Handling
- Invalid deck ID detection
- Network timeout handling
- Rate limiting response handling
- Malformed JSON response handling

## 5. Anki Package Generation (using genanki-rs)

### 5.1 Deck Creation
```rust
use genanki_rs::{Deck, Note};

struct AnkiPackageBuilder {
    deck: Deck,
    model: Model,
    existing_words: HashSet<String>,
}

impl AnkiPackageBuilder {
    fn new(deck_name: &str) -> Self {
        let model = create_vocabulary_model();
        let deck = Deck::new(2059400110, deck_name, "Vocabulary imported from Duocards");
        
        Self {
            deck,
            model,
            existing_words: HashSet::new(),
        }
    }
    
    fn add_note(&mut self, vocab_card: VocabularyCard) -> Result<bool> {
        if self.existing_words.contains(&vocab_card.word) {
            return Ok(false); // Duplicate
        }
        
        let note = VocabularyNote::from(vocab_card).to_anki_note(&self.model);
        self.deck.add_note(note);
        self.existing_words.insert(vocab_card.word.clone());
        Ok(true)
    }
    
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.deck.write_to_file(path.as_ref().to_str().unwrap())
            .map_err(|e| anyhow::anyhow!("Failed to write Anki package: {}", e))
    }
}
```

### 5.2 Simple Implementation
```rust
impl AnkiPackageBuilder {
    fn new(deck_name: &str) -> Self {
        let model = create_vocabulary_model();
        let deck = Deck::new(2059400110, deck_name, "Vocabulary imported from Duocards");
        
        Self {
            deck,
            model,
            existing_words: HashSet::new(), // Tracks duplicates in current session
        }
    }
    
    fn add_note(&mut self, vocab_card: VocabularyCard) -> Result<bool> {
        if self.existing_words.contains(&vocab_card.word) {
            return Ok(false); // Duplicate
        }
        
        let note = VocabularyNote::from(vocab_card).to_anki_note(&self.model);
        self.deck.add_note(note);
        self.existing_words.insert(vocab_card.word.clone());
        Ok(true)
    }
    
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.deck.write_to_file(path.as_ref().to_str().unwrap())
            .map_err(|e| anyhow::anyhow!("Failed to write Anki package: {}", e))
    }
}
```

## 6. Duplicate Handling Requirements

### 6.1 Source Duplicates Only
- Track processed words using `HashSet<String>`
- Case-sensitive comparison
- Skip duplicate entries with console warning
- Continue processing remaining cards

### 6.2 Warning Format
```
WARNING: Duplicate card found in source data: 'word'. Skipping.
```

## 7. Progress Reporting

### 7.1 Console Output Format
For Anki output:
```
Initializing export to 'filename.apkg'...
Processing page 1... done.
Processing page 2... done.
...
Export complete. X cards saved to filename.apkg.
```

For JSON output (both file and stdout):
```
Processing page 1... done.
Processing page 2... done.
...
Export complete. X cards processed.
```

When page limit is specified:
```
Initializing export to 'filename.apkg' (limited to N pages)...
Processing page 1... done.
Processing page 2... done.
...
Export complete. X cards saved to filename.apkg.
```

Note: When using stdout output, progress messages are written to stderr to avoid corrupting the JSON output.

## 9. Security Requirements

### 9.1 Deck ID Handling
- Validate deck ID format before use
- No persistent storage of deck ID

### 9.2 File Handling
- Validate output path is within allowed directories
- Use temporary files with proper cleanup
- Handle file permission errors gracefully

## 10. Performance Requirements

### 10.1 Memory Management
- Stream processing for large datasets
- Efficient duplicate checking with hash sets
- Proper cleanup of temporary resources
- Limited memory footprint (< 100MB typical usage)

### 10.2 Processing Efficiency
- Concurrent HTTP requests where appropriate
- Efficient SQLite operations
- Minimal file I/O operations
- Progress reporting without performance impact

## 11. Cross-Platform Requirements

### 11.1 Binary Distribution
- Single executable file per platform
- No external dependencies required
- Support for Windows, macOS, and Linux
- Static linking where possible

### 11.2 File Path Handling
- Platform-appropriate path separators
- Unicode filename support
- Long path support on Windows
- Case-sensitive filename handling

## 12. Testing Requirements

### 12.1 Unit Tests
- HTTP client mocking
- Duplicate detection logic
- Anki package generation
- Error handling scenarios
- JSON output validation for both file and stdout paths
- Progress message separation for stdout output
- Pipe operation testing
- Page limit validation and enforcement
- Partial export completion with page limits

### 12.2 Integration Tests
- End-to-end transfer simulation
- Error recovery testing
- Cross-platform compatibility
- JSON stdout output with pipe operations
- Progress message handling in pipe scenarios
- Large dataset handling via stdout
- Page limit functionality with various limits
- Partial export completion scenarios

## 13. Build and Deployment

### 13.1 Build Configuration
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### 13.2 Cross-Compilation Targets
- `x86_64-pc-windows-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-unknown-linux-gnu`

## 14. Documentation Requirements

### 14.1 Built-in Help
- Comprehensive `--help` output
- Usage examples in help text
- Clear parameter descriptions

### 14.2 README Documentation
- Installation instructions
- Cookie extraction guide
- Usage examples
- Troubleshooting section

## 15. Output Format Requirements

### 15.1 Anki Package Format
- Generates standard Anki package file (.apkg)
- Uses genanki-rs for package creation
- Maps Duocards fields to Anki note fields:
  - Word → Front
  - Translation → Back
  - Example → Example field
- Converts learning status to Anki tags:
  - new → duoload_new
  - learning → duoload_learning
  - known → duoload_known

### 15.2 JSON Format
- Supports two output destinations through the processor:
  1. File output (--json-file): Writes to specified file path
  2. Standard output (--json): Writes directly to stdout
- Generates UTF-8 encoded JSON
- Array of card objects with the following structure:
```json
[
    {
        "word": "hello",
        "translation": "hallo",
        "example": "Hallo, wie geht's?",
        "learning_status": "new"
    },
    {
        "word": "world",
        "translation": "Welt",
        "example": "Die Welt ist schön.",
        "learning_status": "learning"
    }
]
```
- Pretty-printed for readability
- Progress messages written to stderr when using stdout output

### 15.3 Output Stream Handling
The processor handles output destination selection:

```rust
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
    output_path: Option<PathBuf>,  // None indicates stdout output
}

impl<C, B> TransferProcessorWithBuilder<C, B>
where
    C: DuocardsClientTrait,
    B: OutputBuilder,
{
    pub fn write_to_output(&self) -> Result<()> {
        match &self.output_path {
            Some(path) => {
                // Write to file
                self.builder.write_to_file(path)
            }
            None => {
                // Write to stdout
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                self.builder.write_to_writer(&mut handle)
            }
        }
    }
}
```

The `JsonOutputBuilder` remains unchanged, focusing only on JSON generation and duplicate handling. The processor handles the decision of where to write the output based on the presence of a path.