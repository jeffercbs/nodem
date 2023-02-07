use std::error::Error;

use super::request;
use select::{node::Node, predicate::Name};

fn node_filer(node: Node) -> String {
    node.text().trim().to_lowercase()
}

pub async fn document(url: &str) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let req = request::fetch_document(url)
        .await
        .unwrap()
        .find(Name("a"))
        .map(node_filer)
        .enumerate()
        .collect::<Vec<_>>();

    Ok(req)
}

// pub fn unzip() {}
