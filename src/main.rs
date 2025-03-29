use shiqu::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::from_env().expect("Failed to load config from environment");
    shiqu::run(config).await
}
