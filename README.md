## Overview

Duoload is a command-line tool that allows you to export your data from [Duocards](https://duocards.com/) and transfer it to either Anki flashcards or a JSON format. This gives you full control over your learning data, enabling you to use it with the powerful Anki ecosystem or process it with your own tools.

This application is intended to be used only with user-created cards. Please, respect copyright and don't download own Duocards card stacks.

## Installation

### Binary Installation

Duoload provides pre-built binaries for all major platforms. You can download the latest release from the [GitHub releases page](https://github.com/amarao/duoload/releases):

* Linux (AMD64): `duoload-linux-amd64`
* Linux (ARM64): `duoload-linux-arm64` (broken, under construction)
* Windows (AMD64): `duoload-windows-amd64.exe`
* macOS (AMD64): `duoload-macos-amd64`
* macOS (ARM64): `duoload-macos-arm64`

After downloading, extract archive, make the binary executable (on Unix-like systems):
```bash
unzip duoload-linux-amd64.zip
cd duoload-linux-amd64
chmod +x duoload
./duocard --verion
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

### Finding Your Deck ID

Before using Duoload, you'll need your Duocards deck ID. This is a base64-encoded identifier that looks like: `RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=`

TODO: Instructions on how to find your deck ID in the Duocards application.

### Examples

#### 1. Export to Anki Package

Export your vocabulary to an Anki package file that can be directly imported into Anki:

```bash
# Using binary
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --anki-file "my_vocabulary.apkg"

# Using Docker
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --anki-file "/data/my_vocabulary.apkg"
```

#### 2. Export to JSON File

Save your vocabulary as a JSON file for custom processing:

```bash
# Using binary
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json-file "my_vocabulary.json"

# Using Docker
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json-file "/data/my_vocabulary.json"
```

#### 3. Export to JSON via stdout

Pipe the JSON output directly to other tools or save it to a file:

```bash
# Using binary
# Save to file
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json > my_vocabulary.json

# Process with jq
./duoload --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" --json | jq '.[] | select(.learning_status == "new")'

# Using Docker
# Save to file
docker run --rm -v "$(pwd):/data" ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json > my_vocabulary.json

# Process with jq
docker run --rm ghcr.io/amarao/duoload:latest \
    --deck-id "RGVjazo1YjZmMTA3My1hZjA2LTQwMGMtYTQyNC05ZWM5YzFlMGEzZjg=" \
    --json | jq '.[] | select(.learning_status == "new")'
```

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

It also was tested to work with Duocards as per Jule 2025 with a personal database of 2k+ words.


# Current state and plans

Most of the plan was implemented, we have working code and packages.

Leftovers:

* Fix linux/arm issue for build and image
* Update docs on how to extract deck id. It is not the simplest task.
* I want to validate it works on someone's else machine and deck before final release.
