use anyhow::Result;
use duoload::duocards::DuocardsClient;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <deck_id>", args[0]);
        eprintln!("Example: {} RGVjazo0NmYyYjllZC1hYmYzLTRiZDgtYTA1NC02OGRmYTRhNDIwM2U=", args[0]);
        std::process::exit(1);
    }

    let deck_id = &args[1];
    let client = DuocardsClient::new()?;
    
    // Validate deck ID before making the request
    if let Err(e) = client.validate_deck_id(deck_id) {
        eprintln!("Error: Invalid deck ID: {}", e);
        std::process::exit(1);
    }
    
    let response = client.fetch_page(deck_id, None).await?;
    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
} 