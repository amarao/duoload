Product Requirements Document: duoload

Version: 1.0
Date: June 7, 2025
1. Introduction & Vision

duoload is a local, multi-platform command-line interface (CLI) utility designed to transfer a user's vocabulary data from the Duocards application into either Anki flashcard system or a JSON format. The project empowers users to own their learning data by migrating it from a closed platform to either Anki's open and extensible ecosystem or a portable JSON format for custom processing. The tool is designed for simplicity, reliability, and local execution, ensuring user data privacy.
2. Target Audience

The primary user is a technically experienced computer user who is:

    A language learner using Duocards and either:
        Anki for flashcard-based learning, or
        Custom tools that can process JSON data
    Comfortable running applications from a command line.
    Capable of finding their Duocards deck ID from the application.
    Using a Windows, macOS, or Linux operating system.

3. User Problem

Language learners invest significant time building vocabulary lists in applications like Duocards. However, this data is often locked within the app's ecosystem. Users who wish to consolidate their learning materials into either a single, powerful, and open-source platform like Anki or process their data with custom tools have no direct way to do so. This creates a barrier to long-term, customized learning and data ownership. duoload solves this by bridging the gap between Duocards and both Anki and custom data processing workflows.
4. Goals & Success Metrics

    Primary Goal: To provide a reliable method for users to export their vocabulary from Duocards and import it into Anki.
    Success Metrics:
        Successful execution of the transfer process with clear output.
        Correct generation of an Anki file (.apkg) that can be imported without errors.
        Positive user feedback confirming successful data migration.

5. Features & Functional Requirements
5.1. Data Transfer

    The application will fetch word data from Duocards via its internal API.
    The following data fields will be extracted for each card:
        Word: The foreign language word.
        Translation: The user's native language translation.
        Example of Use: The sentence demonstrating the word's usage.
        Learning Status: The current learning state of the word.
    Image and audio data will not be transferred.
    The data will be processed into one of two output formats:
        Anki package (.apkg) for direct import into Anki
        JSON file containing an array of card objects

5.2. Output Formats

    5.2.1 Anki Integration
        The tool will generate a standard Anki package file (.apkg) that can be imported into the Anki desktop application.
        Data Mapping:
            Duocards Word → Anki Note Front field.
            Duocards Translation → Anki Note Back field.
            Duocards Example of Use → Anki Note Example field.
        Learning Status Conversion: The Duocards status will be converted into an Anki tag on the note.
            new → duoload_new tag
            learning → duoload_learning tag
            known → duoload_known tag

    5.2.2 JSON Output
        The tool will generate JSON output in one of two ways:
            a. JSON File: Write to a specified file path
            b. Standard Output: Write directly to stdout for piping to other tools
        Each card object will have the following structure:
            {
                "word": string,
                "translation": string,
                "example": string,
                "learning_status": "new" | "learning" | "known"
            }
        The JSON output will be UTF-8 encoded and properly formatted for readability.
        When writing to stdout, the output will be a single JSON array containing all cards.

5.4. CLI Functionality

    Deck ID: The user must provide their Duocards deck ID via a command-line argument:
        --deck-id "<deck_id>"
    Output Format: The user must specify exactly one of the following output options:
        --anki-file "path/to/my_deck.apkg" (for Anki format)
        --json-file "path/to/my_deck.json" (for JSON format to file)
        --json (for JSON format to stdout)
    The application will require exactly one output option to be specified.

5.5. Feedback & Error Handling

    Normal Operation: The CLI will provide simple text feedback for each page of data it successfully fetches and processes (e.g., "Processing page 1... done."). No complex Terminal User Interface (TUI) is needed.
    Invalid Deck ID: If the provided deck ID is invalid or doesn't exist, the application will terminate gracefully with a clear error message (e.g., "ERROR: The provided deck ID is invalid or doesn't exist.").
    Connection Errors: For network-related issues (e.g., timeouts, DNS errors), the application will:
        Attempt to reconnect using an exponential backoff strategy.
        Cease retries after a configurable TIMEOUT value.
        Implement a polite delay between all page requests to avoid overwhelming the Duocards server.

6. Non-Functional Requirements

    Platform Support: The application must be a self-contained executable that runs on Windows, macOS, and Linux.
    Usability: CLI commands and arguments must be clear and well-documented. Error messages must be informative and actionable.
    Security: The application does not require any authentication or personal data storage.
    Performance: The primary performance constraint is the polite delay between API calls. The application should be efficient in its use of memory and CPU.
    Language: Rust

7. CLI Design & User Flow

Scenario 1: Export to Anki

The user wants to export their entire collection to a new Anki file.
Bash

duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --anki-file "duocards_export.apkg"

Expected Output:

Initializing new Anki file at 'duocards_export.apkg'...
Processing page 1... done.
Processing page 2... done.
...
Processing page 25... done.
Export complete. 1250 cards saved to duocards_export.apkg.

Scenario 2: Export to JSON File

The user wants to export their collection to a JSON file for custom processing.
Bash

duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json-file "duocards_export.json"

Expected Output:

Initializing JSON export to 'duocards_export.json'...
Processing page 1... done.
Processing page 2... done.
...
Processing page 25... done.
Export complete. 1250 cards saved to duocards_export.json.

Scenario 3: Export to JSON via stdout

The user wants to pipe the JSON output directly to another tool.
Bash

duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json | jq '.[] | select(.learning_status == "new")'

Expected Output:

Processing page 1... done.
Processing page 2... done.
...
Processing page 25... done.
Export complete. 1250 cards processed.
[{"word": "hello", "translation": "hola", "example": "Hello, world!", "learning_status": "new"}, ...]
