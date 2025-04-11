use dotenv::dotenv;
use hetzner::{HetznerClient, records};
use std::env;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("hello world!");
    init_tracing();

    let api_token: String =
        env::var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
    let client: HetznerClient = HetznerClient::new(api_token);

    // let zones: Vec<hetzner::zones::get_all_zones::Zone> = client.get_all_zones().await.unwrap();
    // let zone_id: &str = "xxx";
    // let records = client.get_all_records(zone_id).await.unwrap();

    // let created_record = client
    //     .create_record(
    //         "xxxx",
    //         86400,
    //         "A",
    //         "dexter",
    //         "xxx",
    //     )
    //     .await
    //     .unwrap();

    // println!("{:#?}", created_record);

    let record_id = "215cbf8916dedeeaae4661370dd43b2a";
    let delete_result = client.delete_record(record_id).await;

    info!("Delete result: {:#?}", delete_result);
}

fn init_tracing() {
    let filter: EnvFilter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
