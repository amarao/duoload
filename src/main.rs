use clap::Parser;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();
    let args = Args::parse();

    // Validate that exactly one output format is specified
    if args.anki_file.is_none() && args.json_file.is_none() {
        eprintln!("Error: Please specify either --anki-file or --json-file");
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
    println!("Validating deck ID...");
    if let Err(e) = deck::validate_deck_id(&args.deck_id) {
        eprintln!("Error: Invalid deck ID: {}", e);
        exit(1);
    }

    if let Some(path) = args.anki_file {
        println!("Exporting to Anki package '{:?}'...", path);
        let mut processor = TransferProcessor::new(
            client,
            AnkiPackageBuilder::new("Duocards Vocabulary"),
            args.deck_id
        );
        processor.process_all().await?;
        processor.write_to_file(&path)?;

        // Print success message
        let stats = processor.stats();
        println!("Export completed successfully!");
        println!("Total cards saved: {}", stats.total_cards);
        println!("Duplicates skipped: {}", stats.duplicates);
        println!("Total execution time: {:?}", start_time.elapsed());
    } else {
        let path = args.json_file.unwrap();
        println!("Exporting to JSON file '{:?}'...", path);
        let mut processor = TransferProcessor::new(
            client,
            JsonOutputBuilder::new(),
            args.deck_id
        );
        processor.process_all().await?;
        processor.write_to_file(&path)?;

        // Print success message
        let stats = processor.stats();
        println!("Export completed successfully!");
        println!("Total cards saved: {}", stats.total_cards);
        println!("Duplicates skipped: {}", stats.duplicates);
        println!("Total execution time: {:?}", start_time.elapsed());
    }

    Ok(())
}
