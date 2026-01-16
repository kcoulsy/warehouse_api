use warehouse_api::{Config, server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists (ignores errors if file doesn't exist)
    dotenvy::dotenv().ok();

    let config = Config::from_env();
    server::run_server(config).await
}
