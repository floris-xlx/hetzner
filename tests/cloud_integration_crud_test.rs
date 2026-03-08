//! Full CRUD integration tests: create, list, get, update, delete.
//! Set HETZNER_CLOUD_API_TOKEN to run.

use hetzner::{CreateServerInput, HetznerClient};
use serde_json::json;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn api_token() -> Option<String> {
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.local").ok();
    env::var("HETZNER_CLOUD_API_TOKEN").ok()
}

/// Nanosecond-precision suffix to avoid collisions when tests run in parallel
fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

#[tokio::test]
async fn test_network_crud() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let suffix = unique_suffix();
    let name = format!("test-net-{suffix}");
    let name_updated = format!("test-net-upd-{suffix}");

    let create_body = json!({
        "name": name,
        "ip_range": "10.0.0.0/16",
        "subnets": [{"type": "cloud", "network_zone": "eu-central", "ip_range": "10.0.1.0/24"}]
    });
    let created = client.cloud().private_networks().create(create_body).await.expect("create network");
    let id = created.get("network").and_then(|n| n.get("id")).and_then(|i| i.as_u64()).expect("network id");

    let list: serde_json::Value = client.cloud().list_networks(None, None).await.expect("list networks");
    assert!(list.get("networks").and_then(|n| n.as_array()).unwrap().iter().any(|s| s.get("id").and_then(|i| i.as_u64()) == Some(id)));

    let got: serde_json::Value = client.cloud().private_networks().get(id).await.expect("get network");
    assert_eq!(got.get("network").and_then(|n| n.get("name")).and_then(|n| n.as_str()), Some(name.as_str()));

    let _: serde_json::Value = client.cloud().private_networks().update(id, json!({"name": name_updated})).await.expect("update network");
    let _: serde_json::Value = client.cloud().private_networks().delete(id).await.expect("delete network");
}

#[tokio::test]
async fn test_volume_crud() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let suffix = unique_suffix();
    let name = format!("test-vol-{suffix}");
    let name_updated = format!("test-vol-upd-{suffix}");

    let create_body = json!({"name": name, "size": 10, "location": "fsn1"});
    let created = client.cloud().storage().create_volume(create_body).await.expect("create volume");
    let id = created.get("volume").and_then(|v| v.get("id")).and_then(|i| i.as_u64()).expect("volume id");

    let list: serde_json::Value = client.cloud().list_volumes(None, None).await.expect("list volumes");
    assert!(list.get("volumes").and_then(|v| v.as_array()).unwrap().iter().any(|v| v.get("id").and_then(|i| i.as_u64()) == Some(id)));

    let got: serde_json::Value = client.cloud().storage().get_volume(id).await.expect("get volume");
    assert_eq!(got.get("volume").and_then(|v| v.get("name")).and_then(|n| n.as_str()), Some(name.as_str()));

    let _: serde_json::Value = client.cloud().storage().update_volume(id, json!({"name": name_updated})).await.expect("update volume");
    let _: serde_json::Value = client.cloud().storage().delete_volume(id).await.expect("delete volume");
}

#[tokio::test]
async fn test_ssh_key_crud() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let suffix = unique_suffix();
    let name = format!("test-key-{suffix}");
    let name_updated = format!("test-key-upd-{suffix}");
    let public_key = format!("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl test-{}@example.com", suffix);

    let create_body = json!({"name": name, "public_key": public_key});
    let created = client.cloud().create_ssh_key(None, Some(create_body)).await.expect("create ssh key");
    let id = created.get("ssh_key").and_then(|k| k.get("id")).and_then(|i| i.as_u64()).expect("ssh key id");

    let list: serde_json::Value = client.cloud().list_ssh_keys(None, None).await.expect("list ssh keys");
    assert!(list.get("ssh_keys").and_then(|k| k.as_array()).unwrap().iter().any(|k| k.get("id").and_then(|i| i.as_u64()) == Some(id)));

    let got: serde_json::Value = client.cloud().get_ssh_key(id, None, None).await.expect("get ssh key");
    assert_eq!(got.get("ssh_key").and_then(|k| k.get("name")).and_then(|n| n.as_str()), Some(name.as_str()));

    let _: serde_json::Value = client.cloud().update_ssh_key(id, None, Some(json!({"name": name_updated}))).await.expect("update ssh key");
    let _: serde_json::Value = client.cloud().delete_ssh_key(id, None, None).await.expect("delete ssh key");
}

#[tokio::test]
async fn test_server_crud() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let suffix = unique_suffix();
    let name = format!("test-srv-{suffix}");
    let name_updated = format!("test-srv-upd-{suffix}");

    let created = client.cloud().servers().create(&CreateServerInput {
        name: name.clone(),
        server_type: "cpx22".to_string(),
        image: "ubuntu-22.04".to_string(),
        location: Some("fsn1".to_string()),
        start_after_create: Some(false),
    }).await.expect("create server");
    let server_id = created.server.id;

    for _ in 0..24 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let got = client.cloud().servers().get(server_id).await.expect("get server");
        if got.server.status != hetzner::ServerStatus::Initializing {
            break;
        }
    }

    let servers = client.cloud().servers().list(Some(&hetzner::ListServersParams { name: Some(name.clone()), ..Default::default() })).await.expect("list servers");
    assert!(servers.iter().any(|s| s.id == server_id));

    let got = client.cloud().servers().get(server_id).await.expect("get server");
    assert_eq!(got.server.name, name);

    let _: serde_json::Value = client.cloud().servers_api().update(server_id, json!({"name": name_updated})).await.expect("update server");
    let _: serde_json::Value = client.cloud().servers_api().delete(server_id).await.expect("delete server");
}
