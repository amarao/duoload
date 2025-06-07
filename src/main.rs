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
    #[arg(long, value_name = "COOKIE", help = "Duocards session cookie (copy from the browser after login)")]
    cookie: String,
    
    #[arg(long, value_name = "FILE", help = "file to create (Anki database)")]
    output_file: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Initializing new Anki file at '{}'...", args.output_file.display());

    // // Initialize components
    // let client = DuocardsClient::new(&args.cookie);

    // // Validate authentication
    // if let Err(e) = client.validate_auth().await {
    //     eprintln!("Error: Authentication failed: {}", e);
    //     exit(1);
    // }

    // let mut processor = TransferProcessor::new(
    //     client,
    //     AnkiPackageBuilder::new("Duocards Vocabulary"),
    // );

    // // Process all pages
    // match processor.process_all().await {
    //     Ok(()) => {
    //         println!("Export complete. Cards saved to {}.", args.output_file.display());
    //     }
    //     Err(e) => {
    //         eprintln!("Error during transfer: {}", e);
    //         exit(1);
    //     }
    // }
}
