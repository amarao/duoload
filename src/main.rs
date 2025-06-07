use clap::Parser;
use std::path::PathBuf;

mod duocards;
mod anki;
mod transfer;
mod error;

#[derive(Parser)]
#[command(name = "duoload")]
#[command(about = "Transfer vocabulary from Duocards to Anki")]
struct Args {
    #[arg(long, value_name = "COOKIE", help = "Duocards session cookie")]
    cookie: String,
    
    #[arg(long, value_name = "FILE", help = "Output Anki package file path")]
    output_file: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _args = Args::parse();
    println!("Initializing Duoload...");
    // TODO: Implement main logic
    Ok(())
}
