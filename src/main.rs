use edupage_rs::edupage::Edupage;

#[tokio::main]
async fn main() {
    let mut edpg = Edupage::new();
    let _data = edpg
        .login(
            "skolasvr".to_string(),
            "JakubTulek".to_string(),
            "2x3xrx1m6t".to_string(),
        )
        .await
        .unwrap();
}
