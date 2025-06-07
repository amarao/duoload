use reqwest::Client;
use crate::error::{Result, DuoloadError};
use crate::duocards::models::{DuocardsResponse, VocabularyCard};

pub struct DuocardsClient {
    client: Client,
    cookie: String,
    base_url: String,
}

impl DuocardsClient {
    pub fn new(_cookie: &str) -> Self {
        unimplemented!("DuocardsClient::new")
    }

    pub async fn fetch_page(&self, _page: u32) -> Result<DuocardsResponse> {
        unimplemented!("DuocardsClient::fetch_page")
    }

    pub async fn validate_auth(&self) -> Result<bool> {
        unimplemented!("DuocardsClient::validate_auth")
    }
}
