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
    
    let response = client.fetch_page(deck_id, None).await?;
    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
} 