use shiqu::Config;

#[tokio::test]
async fn test_bot() {
    let config = Config {
        telegram_token: std::env::var("TELEGRAM_TOKEN").unwrap(),
        database_path: "./tests/test.db".to_string(),
        api_key: shiqu::ApiKeyConfig {
            gemini_key: None,
        },
    };
    shiqu::run(config).await;
}
