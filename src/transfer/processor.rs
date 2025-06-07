use crate::error::Result;
use crate::duocards::DuocardsClient;
use crate::anki::AnkiPackageBuilder;
use crate::transfer::DuplicateHandler;

#[derive(Debug)]
pub struct TransferStats {
    pub total_cards: usize,
    pub duplicates: usize,
}

pub struct TransferProcessor {
    client: DuocardsClient,
    builder: AnkiPackageBuilder,
    duplicates: DuplicateHandler,
}

impl TransferProcessor {
    pub fn new(_client: DuocardsClient, _builder: AnkiPackageBuilder) -> Self {
        unimplemented!("TransferProcessor::new")
    }

    pub async fn process_all(&mut self) -> Result<()> {
        unimplemented!("TransferProcessor::process_all")
    }
}
