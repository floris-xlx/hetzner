use hetzner::HetznerClient;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("hello world!");

    let api_token: String = env::var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
    let client: HetznerClient = HetznerClient::new(api_token);

    let zones = client.get_all_zones().await.unwrap();

    println!("{:#?}", zones);
}
