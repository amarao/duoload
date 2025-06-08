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
use duocards::DuocardsClient;
use duocards::deck;
use error::Result;
use transfer::processor::TransferProcessor;

#[derive(Parser)]
#[command(name = "duoload")]
#[command(about = "Transfer vocabulary from Duocards to Anki")]
struct Args {
    #[arg(
        long,
        value_name = "DECK_ID",
        help = "Duocards deck ID (base64 encoded Deck:UUID)"
    )]
    deck_id: String,

    #[arg(long, value_name = "FILE", help = "file to create (Anki database)")]
    output_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();
    let args = Args::parse();

    let client = match DuocardsClient::new() {
        Ok(client) => {
            client
        }
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

    let builder = AnkiPackageBuilder::new("Duocards Vocabulary");
    let mut processor = TransferProcessor::new(client, builder, args.deck_id.clone());

    processor.process_all().await?;

    processor.write_to_file(&args.output_file)?;
    println!("File written at {:?}", start_time.elapsed());

    // Print success message
    let stats = processor.stats();
    println!("Export completed successfully!");
    println!("Total cards saved: {}", stats.total_cards);
    println!("Duplicates skipped: {}", stats.duplicates);
    println!("Total execution time: {:?}", start_time.elapsed());

    Ok(())
}
