use clap::Parser;
use std::path::PathBuf;
use std::process::exit;

mod duocards;
mod anki;
mod transfer;
mod error;

use error::{DuoloadError, Result};
use duocards::DuocardsClient;
use anki::AnkiPackageBuilder;
use transfer::processor::TransferProcessor;

#[derive(Parser)]
#[command(name = "duoload")]
#[command(about = "Transfer vocabulary from Duocards to Anki")]
struct Args {
    #[arg(long, value_name = "DECK_ID", help = "Duocards deck ID (base64 encoded Deck:UUID)")]
    deck_id: String,
    
    #[arg(long, value_name = "FILE", help = "file to create (Anki database)")]
    output_file: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Initializing new Anki file at '{}'...", args.output_file.display());

    // Initialize components
    let client = match DuocardsClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error: Failed to initialize client: {}", e);
            exit(1);
        }
    };

    // Validate deck ID
    if let Err(e) = client.validate_deck_id(&args.deck_id) {
        eprintln!("Error: Invalid deck ID: {}", e);
        exit(1);
    }

    let mut processor = TransferProcessor::new(
        client,
        AnkiPackageBuilder::new("Duocards Vocabulary"),
    );

    // Process all pages
    match processor.process_all().await {
        Ok(()) => {
            println!("Export complete. Cards saved to {}.", args.output_file.display());
        }
        Err(e) => {
            eprintln!("Error during transfer: {}", e);
            exit(1);
        }
    }
}
