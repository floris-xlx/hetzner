use dotenv::dotenv;
use hetzner::{HetznerClient, ListServersParams};
use std::env::var;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let api_token = var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
    let client = HetznerClient::new(api_token);

    match client
        .cloud()
        .servers()
        .list(Some(&ListServersParams::default()))
        .await
    {
        Ok(servers) => info!("servers: {servers:#?}"),
        Err(err) => info!("failed to list servers: {err}"),
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
