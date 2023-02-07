use reqwest::Client;
use select::document::Document;
use std::error::Error;

pub(crate) async fn fetch_document(url: &str) -> Result<Document, Box<dyn Error>> {
    let client = Client::new();
    let req = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .map(|body| Document::from(body.as_ref()))
        .unwrap();

    Ok(req)
}
