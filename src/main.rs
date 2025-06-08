use clap::Parser;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;

mod anki;
mod duocards;
mod error;
mod transfer;

use anki::AnkiPackageBuilder;
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

    println!(
        "Initializing new Anki file at '{}'...",
        args.output_file.display()
    );
    println!(
        "[DEBUG] Starting initialization at {:?}",
        start_time.elapsed()
    );

    // Initialize components
    println!("[DEBUG] Creating DuocardsClient...");
    let client = match DuocardsClient::new() {
        Ok(client) => {
            println!(
                "[DEBUG] DuocardsClient created successfully at {:?}",
                start_time.elapsed()
            );
            client
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize client: {}", e);
            exit(1);
        }
    };

    // Validate deck ID
    println!("[DEBUG] Validating deck ID...");
    if let Err(e) = deck::validate_deck_id(&args.deck_id) {
        eprintln!("Error: Invalid deck ID: {}", e);
        exit(1);
    }
    println!("[DEBUG] Deck ID validated at {:?}", start_time.elapsed());

    println!("[DEBUG] Creating AnkiPackageBuilder...");
    let builder = AnkiPackageBuilder::new("Duocards Vocabulary");
    println!(
        "[DEBUG] AnkiPackageBuilder created at {:?}",
        start_time.elapsed()
    );

    println!("[DEBUG] Creating TransferProcessor...");
    let mut processor = TransferProcessor::new(client, builder, args.deck_id.clone());
    println!(
        "[DEBUG] TransferProcessor created at {:?}",
        start_time.elapsed()
    );

    println!("[DEBUG] Starting card processing...");
    processor.process_all().await?;
    println!(
        "[DEBUG] Card processing completed at {:?}",
        start_time.elapsed()
    );

    // Write the output file
    println!("[DEBUG] Writing to file...");
    processor.write_to_file(&args.output_file)?;
    println!("[DEBUG] File written at {:?}", start_time.elapsed());

    // Print success message
    let stats = processor.stats();
    println!("Export completed successfully!");
    println!("Total cards saved: {}", stats.total_cards);
    println!("Duplicates skipped: {}", stats.duplicates);
    println!("[DEBUG] Total execution time: {:?}", start_time.elapsed());

    Ok(())
}
