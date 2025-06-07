Product Requirements Document: duoload

Version: 1.0
Date: June 7, 2025
1. Introduction & Vision

duoload is a local, multi-platform command-line interface (CLI) utility designed to transfer a user's vocabulary data from the Duocards application into the Anki flashcard system. The project empowers users to own their learning data by migrating it from a closed platform to Anki's open and extensible ecosystem. The tool is designed for simplicity, reliability, and local execution, ensuring user data privacy.
2. Target Audience

The primary user is a technically experienced computer user who is:

    A language learner using both Duocards and Anki.
    Comfortable running applications from a command line.
    Capable of retrieving a web cookie from their browser's developer console.
    Using a Windows, macOS, or Linux operating system.

3. User Problem

Language learners invest significant time building vocabulary lists in applications like Duocards. However, this data is often locked within the app's ecosystem. Users who wish to consolidate their learning materials into a single, powerful, and open-source platform like Anki have no direct way to do so. This creates a barrier to long-term, customized learning and data ownership. duoload solves this by bridging the gap between Duocards and Anki.
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

5.2. Anki Integration

    The tool will generate a standard Anki package file (.apkg) that can be imported into the Anki desktop application.
    Data Mapping:
        Duocards Word → Anki Note Front field.
        Duocards Translation → Anki Note Back field.
        Duocards Example of Use → Anki Note Example field.
    Learning Status Conversion: The Duocards status will be converted into an Anki tag on the note.
        new → duoload_new tag
        learning → duoload_learning tag
        known → duoload_known tag

5.4. CLI Functionality

    Authentication: The user must provide their Duocards session cookie via a command-line argument:
        --cookie "<cookie_string>"
    Output File: The user must specify the path for the output Anki file:
        --output-file "path/to/my_deck.apkg"

5.5. Feedback & Error Handling

    Normal Operation: The CLI will provide simple text feedback for each page of data it successfully fetches and processes (e.g., "Processing page 1... done."). No complex Terminal User Interface (TUI) is needed.
    Invalid Cookie: If the provided cookie is invalid or expired, the application will terminate gracefully with a clear error message (e.g., "ERROR: The provided cookie is invalid or has expired.").
    Connection Errors: For network-related issues (e.g., timeouts, DNS errors), the application will:
        Attempt to reconnect using an exponential backoff strategy.
        Cease retries after a configurable TIMEOUT value.
        Implement a polite delay between all page requests to avoid overwhelming the Duocards server and appear as a normal client.

6. Non-Functional Requirements

    Platform Support: The application must be a self-contained executable that runs on Windows, macOS, and Linux.
    Usability: CLI commands and arguments must be clear and well-documented. Error messages must be informative and actionable.
    Security: The application will not store the user's cookie or any personal data. The cookie will only be held in memory during the application's runtime.
    Performance: The primary performance constraint is the polite delay between API calls. The application should be efficient in its use of memory and CPU.
    Language: Rust

7. CLI Design & User Flow

Scenario 1: New Export

The user wants to export their entire collection to a new file.
Bash

duoload --cookie "sessionid=xyz; ..." --output-file "duocards_export.apkg"

Expected Output:

Initializing new Anki file at 'duocards_export.apkg'...
Processing page 1... done.
Processing page 2... done.
...
Processing page 25... done.
Export complete. 1250 cards saved to duocards_export.apkg.
