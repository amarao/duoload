use clap::Parser;
use std::path::PathBuf;
use std::process::exit;

mod anki;
mod duocards;
mod error;
mod output;
mod transfer;

use crate::output::anki::AnkiPackageBuilder;
use crate::output::json::JsonOutputBuilder;
use duocards::DuocardsClient;
use duocards::deck;
use error::Result;
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Validate that exactly one output format is specified
    if args.anki_file.is_none() && args.json_file.is_none() && !args.json {
        eprintln!("Error: Please specify either --anki-file, --json-file, or --json");
        exit(1);
    }

    let client = match DuocardsClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error: Failed to initialize client: {}", e);
            exit(1);
        }
    };

    // Validate deck ID
    eprintln!("Validating deck ID...");
    if let Err(e) = deck::validate_deck_id(&args.deck_id) {
        eprintln!("Error: Invalid deck ID: {}", e);
        exit(1);
    }

    let processor = TransferProcessor::new(client, args.deck_id);

    if let Some(path) = args.anki_file {
        eprintln!("Exporting to Anki package '{:?}'...", path);
        let mut processor = processor.output(AnkiPackageBuilder::new("Duocards Vocabulary"), path);
        processor.process().await?;
    } else if args.json {
        eprintln!("Exporting to stdout...");
        let mut processor = processor.output(JsonOutputBuilder::new(), PathBuf::from("-"));
        processor.process().await?;
    } else {
        let path = args.json_file.unwrap();
        eprintln!("Exporting to JSON file '{:?}'...", path);
        let mut processor = processor.output(JsonOutputBuilder::new(), path);
        processor.process().await?;
    }

    Ok(())
}
