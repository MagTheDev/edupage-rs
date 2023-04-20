use std::collections::HashMap;

use edupage_rs::{models::Teacher, edupage::Edupage};

#[tokio::main]
async fn main() {
    let mut edpg = Edupage::new();
    let data = edpg.login("skolasvr".to_string(), "JakubTulek".to_string(), "2x3xrx1m6t".to_string()).await.unwrap();

}