use clap::Parser;
use std::path::PathBuf;

mod anki;
mod duocards;
mod error;
mod output;
mod transfer;

use crate::output::anki::AnkiPackageBuilder;
use crate::output::json::JsonOutputBuilder;
use duocards::DuocardsClient;
use duocards::deck;
use error::{DuoloadError, Result};
use transfer::processor::TransferProcessor;

#[derive(Parser)]
#[command(name = "duoload")]
#[command(about = "Transfer vocabulary from Duocards to Anki or JSON")]
struct Args {
    #[arg(
        long,
        value_name = "DECK_ID",
        help = "Duocards deck ID (base64 encoded Deck:UUID)"
    )]
    deck_id: String,

    #[arg(
        long,
        value_name = "FILE",
        help = "Output Anki package file (.apkg)",
        group = "output_format"
    )]
    anki_file: Option<PathBuf>,

    #[arg(
        long,
        value_name = "FILE",
        help = "Output JSON file (.json)",
        group = "output_format"
    )]
    json_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Output JSON to stdout (for piping to other tools)",
        group = "output_format"
    )]
    json: bool,

    #[arg(
        long,
        value_name = "N",
        help = "Limit export to N pages (default: all pages)",
        value_parser = validate_page_limit
    )]
    pages: Option<u32>,
}

/// Validate that the page limit is a positive integer
fn validate_page_limit(s: &str) -> std::result::Result<u32, String> {
    match s.parse::<u32>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err("Page limit must be a positive integer".to_string()),
        Err(_) => Err("Page limit must be a valid positive integer".to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Validate that exactly one output format is specified
    if args.anki_file.is_none() && args.json_file.is_none() && !args.json {
        return Err(DuoloadError::Api(
            "Please specify either --anki-file, --json-file, or --json".to_string(),
        ));
    }

    let mut client = match DuocardsClient::new() {
        Ok(client) => client,
        Err(e) => {
            return Err(DuoloadError::Api(format!(
                "Failed to initialize client: {}",
                e
            )));
        }
    };

    // Set page limit if specified
    if let Some(limit) = args.pages {
        client = client.with_page_limit(limit);
    }

    // Validate deck ID
    eprintln!("Validating deck ID...");
    if let Err(e) = deck::validate_deck_id(&args.deck_id) {
        return Err(DuoloadError::Api(format!("Invalid deck ID: {}", e)));
    }

    let processor = TransferProcessor::new(client, args.deck_id);

    if let Some(path) = args.anki_file {
        if let Some(limit) = args.pages {
            eprintln!(
                "Exporting to Anki package '{:?}' (limited to {} pages)...",
                path, limit
            );
        } else {
            eprintln!("Exporting to Anki package '{:?}'...", path);
        }
        let mut processor = processor.output(AnkiPackageBuilder::new("Duocards Vocabulary"), path);
        processor.process().await?;
    } else if args.json {
        if let Some(limit) = args.pages {
            eprintln!("Exporting to stdout (limited to {} pages)...", limit);
        } else {
            eprintln!("Exporting to stdout...");
        }
        let mut processor = processor.output(JsonOutputBuilder::new(), PathBuf::from("-"));
        processor.process().await?;
    } else {
        let path = args.json_file.unwrap();
        if let Some(limit) = args.pages {
            eprintln!(
                "Exporting to JSON file {:?} (limited to {} pages)...",
                path, limit
            );
        } else {
            eprintln!("Exporting to JSON file {:?}...", path);
        }
        let mut processor = processor.output(JsonOutputBuilder::new(), path);
        processor.process().await?;
    }

    Ok(())
}
