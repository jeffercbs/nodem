use regex::Regex;

use super::get;

const NODE_ADDRESS: &str = "https://nodejs.org/dist";

pub async fn available(r: String) {
    if !r.is_empty() {
        let regex = Regex::new(r"^v\d+\.\d+\.\d+/").unwrap();
        let versions = get::document(NODE_ADDRESS).await.unwrap();

        for (_, v) in versions.iter() {
            if regex.is_match(v.as_str()) {
                println!("{}", v)
            }
        }
    } else {
        println!("list")
    }
}

pub async fn download(v: String) {
    let url = format!("{}/v{}", NODE_ADDRESS, v);
    let v = get::document(url.as_str()).await.unwrap();

    for (_, v) in v.iter() {
        println!("{}", v)
    }
    println!("Download {}", url)
}
