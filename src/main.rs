use dotenv::dotenv;
// use hetzner::{HetznerClient, records};
// use std::env;
// use tracing::info;
// use tracing_subscriber::EnvFilter;

// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//     println!("hello world!");
//     init_tracing();

//     let api_token: String =
//         env::var("HETZNER_API_ACCESS_TOKEN").expect("HETZNER_API_ACCESS_TOKEN must be set");
//     let client: HetznerClient = HetznerClient::new(api_token);

//     let zones: Vec<hetzner::Zone> = client.get_all_zones().await.unwrap();
//     info!("zones: {:#?}", zones);



//     let zone_id: &str = "Sy37QEdpZ5nUD6YFTLHMmZ";
//     let records = client.get_all_records(zone_id).await.unwrap();

//     info!("records: {:#?}", records);

//     // let created_record = client
//     //     .create_record("65.108.104.231", 86400, "A", "dexter", zone_id)
//     //     .await
//     //     .unwrap();

//     // println!("{:#?}", created_record);

//     // let record_id = "215cbf8916dedeeaae4661370dd43b2a";
//     // let delete_result = client.delete_record(record_id).await;

//     // info!("Delete result: {:#?}", delete_result);

//     // update record
//     // let updated_record = client
//     //     .update_record(
//     //         "abbacd389349654544053a4a8364f4c9",
//     //         zone_id,
//     //         "A",
//     //         "dexter",
//     //         "65.108.104.231",
//     //         86400,
//     //     )
//     //     .await;

//     // info!("Updated record: {:#?}", updated_record);


//     // let get_record = client
//     //     .get_record("abbacd389349654544053a4a8364f4c9")
//     //     .await
//     //     .unwrap();


//     // info!("Get record: {:#?}", get_record);
// }

// fn init_tracing() {
//     let filter: EnvFilter =
//         EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

//     tracing_subscriber::fmt().with_env_filter(filter).init();
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
}
