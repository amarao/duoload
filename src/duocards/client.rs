use reqwest::Client;
use crate::error::Result;
use super::models::DuocardsResponse;

pub struct DuocardsClient {
    _client: Client,
    _cookie: String,
    _base_url: String,
}

impl DuocardsClient {
    pub fn new(_cookie: String) -> Self {
        unimplemented!("DuocardsClient::new")
    }

    pub async fn fetch_page(&self, _page: u32) -> Result<DuocardsResponse> {
        unimplemented!("DuocardsClient::fetch_page")
    }

    pub async fn validate_auth(&self) -> Result<bool> {
        unimplemented!("DuocardsClient::validate_auth")
    }
}
