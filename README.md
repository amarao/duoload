## Overview

Duoload is a command-line tool that allows you to export your data from [Duocards](https://duocards.com/) and transfer it to either Anki flashcards or a JSON format. This gives you full control over your learning data, enabling you to use it with the powerful Anki ecosystem or process it with your own tools.

This application is intended to be used only with user-created cards. Please, respect copyright and don't download own Duocards card stacks.

## Installation

There are three ways to get Duoload:

* Download an archive with pre-compiled binary
* Build your own from source code
* Use Docker image

### Binary Installation

Duoload provides pre-built binaries for all major platforms. You can download the latest release from the [GitHub releases page](https://github.com/amarao/duoload/releases):

* Linux (AMD64): `duoload-linux-amd64`
* Linux (ARM64): `duoload-linux-arm64`
* Windows (AMD64): `duoload-windows-amd64.exe`
* macOS (AMD64): `duoload-macos-amd64`
* macOS (ARM64): `duoload-macos-arm64`

Note: all binaries are coming unsinged (I don't have excess €100 to throw to Apple just for privilege to compile opensource software for
their OS).

After downloading, extract archive, make the binary executable (on Unix-like systems):
```bash
unzip duoload-linux-amd64.zip
cd duoload-linux-amd64
chmod +x duoload
./duocard --verion
```

### Build your own

Run

```
cargo install duoload
```

### Docker

Duoload is also available as a Docker image. You can pull it from GitHub Container Registry:

```bash
# Pull latest version
docker pull ghcr.io/amarao/duoload:latest

# Or pull specific version
docker pull ghcr.io/amarao/duoload:v1.0.0
```

## Usage

To download your vocabulary you need to know deck ID in Duocards. Deck ID is a base64-encoded identifier that looks like: `RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=`

### Finding Your Deck ID

Instructions for Firefox:

* Login into https://app.duocards.com
* Open your browser debug console (usually, F12)
* Go to into storage tab (1 on the picture)
* Go into 'Local Storage' (2 on the picture)
* Find 'reduxState' line, click on it. (3 on the picture)
* On the right pannel find 'deckID' (scroll manually or put it into search line on the right, 4 on the picture)
* Click on this value (on the right pannel) (5 on the picture)
* Copy value (just press Ctrl-C or Cmd-C)
* Remove 'deckID:' from copied string. Everything after column is deck ID to use in command line with duoload.

![Deck ID](deck_id.png)

### Examples

#### 1. Export to Anki Package

Export your vocabulary to an Anki package file that can be directly imported into Anki:

```bash
# Using binary - export all pages
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --anki-file "my_vocabulary.apkg"

# Using binary - export only first 5 pages
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --anki-file "my_vocabulary.apkg" --pages 5

# Using Docker - export all pages
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --anki-file "/data/my_vocabulary.apkg"

# Using Docker - export only first 3 pages
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --anki-file "/data/my_vocabulary.apkg" \
    --pages 3
```

#### 2. Export to JSON File

Save your vocabulary as a JSON file for custom processing:

```bash
# Using binary - export all pages
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json-file "my_vocabulary.json"

# Using binary - export only first 10 pages
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json-file "my_vocabulary.json" --pages 10

# Using Docker - export all pages
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json-file "/data/my_vocabulary.json"

# Using Docker - export only first 5 pages
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json-file "/data/my_vocabulary.json" \
    --pages 5
```

#### 3. Export to JSON via stdout

Pipe the JSON output directly to other tools or save it to a file:

```bash
# Using binary - export all pages
# Save to file
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json > my_vocabulary.json

# Using binary - export only first 2 pages
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json --pages 2 > my_vocabulary.json

# Process with jq - export all pages
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json | jq '.[] | select(.learning_status == "new")'

# Process with jq - export only first 3 pages
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json --pages 3 | jq '.[] | select(.learning_status == "new")'

# Using Docker - export all pages
# Save to file
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json > my_vocabulary.json

# Using Docker - export only first 5 pages
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json --pages 5 > my_vocabulary.json

# Process with jq - export all pages
docker run --rm ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json | jq '.[] | select(.learning_status == "new")'

# Process with jq - export only first 2 pages
docker run --rm ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json --pages 2 | jq '.[] | select(.learning_status == "new")'
```

### Command Line Options

The following options are available:

- `--deck-id`: (Required) Your Duocards deck ID
- `--anki-file`: Output path for Anki package (.apkg)
- `--json-file`: Output path for JSON file
- `--json`: Output JSON to stdout (for piping to other tools)
- `--pages`: (Optional) Limit export to N pages (default: all pages)

Note: You must specify exactly one output format (either `--anki-file`, `--json-file`, or `--json`).

## Output Format

### Anki Package (.apkg)
The generated Anki package contains your vocabulary cards with the following fields:
- Front: The foreign language word
- Back: The translation
- Example: Example usage (if available)
- Tags: Learning status (duoload_new, duoload_learning, or duoload_known)

### JSON Format
The JSON output is an array of card objects with the following structure:
```json
[
    {
        "word": "hello",
        "translation": "hallo",
        "example": "Hallo, wie geht's?",
        "learning_status": "new"
    }
]
```


## Vibe coding

This utility was vibe coded using:

* Gemini 2.5 Pro
* Cloudie 4.0 sonnet
* Cursor using 'auto' mode

It also got some manual polishing (mostly in specs, PRDs and tests) and was completely reviewed by humans.

It also was tested to work with Duocards as per June 2025 with a personal database of 2k+ words.
