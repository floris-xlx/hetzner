//! Integration tests against the real Hetzner Cloud API.
//! Set HETZNER_CLOUD_API_TOKEN to run these tests.

use hetzner::{CreateServerInput, HetznerClient};
use serde_json::json;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn api_token() -> Option<String> {
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.local").ok();
    env::var("HETZNER_CLOUD_API_TOKEN").ok()
}

fn unique_suffix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[tokio::test]
async fn test_list_servers() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().servers_api().list(None).await;
    assert!(result.is_ok(), "list servers: {:?}", result.err());
}

#[tokio::test]
async fn test_list_locations() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_locations(None, None).await;
    assert!(result.is_ok(), "list locations: {:?}", result.err());
}

#[tokio::test]
async fn test_list_datacenters() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_datacenters(None, None).await;
    assert!(result.is_ok(), "list datacenters: {:?}", result.err());
}

#[tokio::test]
async fn test_list_server_types() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_server_types(None, None).await;
    assert!(result.is_ok(), "list server types: {:?}", result.err());
}

#[tokio::test]
async fn test_list_images() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_images(None, None).await;
    assert!(result.is_ok(), "list images: {:?}", result.err());
}

#[tokio::test]
async fn test_list_networks() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_networks(None, None).await;
    assert!(result.is_ok(), "list networks: {:?}", result.err());
}

#[tokio::test]
async fn test_list_volumes() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_volumes(None, None).await;
    assert!(result.is_ok(), "list volumes: {:?}", result.err());
}

#[tokio::test]
async fn test_list_load_balancers() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_load_balancers(None, None).await;
    assert!(result.is_ok(), "list load balancers: {:?}", result.err());
}

#[tokio::test]
async fn test_list_zones() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_zones(None, None).await;
    assert!(result.is_ok(), "list zones: {:?}", result.err());
}

#[tokio::test]
async fn test_list_ssh_keys() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().list_ssh_keys(None, None).await;
    assert!(result.is_ok(), "list ssh keys: {:?}", result.err());
}

#[tokio::test]
async fn test_get_pricing() {
    let token = match api_token() {
        Some(t) => t,
        None => return,
    };
    let client = HetznerClient::new(token);
    let result = client.cloud().get_pricing(None, None).await;
    assert!(result.is_ok(), "get pricing: {:?}", result.err());
}

// --- CRUD tests: create, list, get, update, delete ---

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

    // Create
    let create_body = json!({
        "name": name,
        "ip_range": "10.0.0.0/16",
        "subnets": [{"type": "cloud", "network_zone": "eu-central", "ip_range": "10.0.1.0/24"}]
    });
    let created = client
        .cloud()
        .private_networks()
        .create(create_body)
        .await
        .expect("create network");
    let net = created.get("network").and_then(|n| n.get("id"));
    let id = net.and_then(|i| i.as_u64()).expect("network id in response");

    // List and verify
    let list: serde_json::Value = client
        .cloud()
        .list_networks(None, None)
        .await
        .expect("list networks");
    let servers = list.get("networks").and_then(|n| n.as_array()).unwrap();
    assert!(
        servers.iter().any(|s| s.get("id").and_then(|i| i.as_u64()) == Some(id)),
        "created network in list"
    );

    // Get
    let got: serde_json::Value = client
        .cloud()
        .private_networks()
        .get(id)
        .await
        .expect("get network");
    assert_eq!(got.get("network").and_then(|n| n.get("name")).and_then(|n| n.as_str()), Some(name.as_str()));

    // Update
    let update_body = json!({"name": name_updated});
    let _: serde_json::Value = client
        .cloud()
        .private_networks()
        .update(id, update_body)
        .await
        .expect("update network");

    // Delete
    let _: serde_json::Value = client
        .cloud()
        .private_networks()
        .delete(id)
        .await
        .expect("delete network");
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

    // Create
    let create_body = json!({
        "name": name,
        "size": 10,
        "location": "fsn1"
    });
    let created = client
        .cloud()
        .storage()
        .create_volume(create_body)
        .await
        .expect("create volume");
    let vol = created.get("volume").and_then(|v| v.get("id"));
    let id = vol.and_then(|i| i.as_u64()).expect("volume id in response");

    // List and verify
    let list: serde_json::Value = client
        .cloud()
        .list_volumes(None, None)
        .await
        .expect("list volumes");
    let volumes = list.get("volumes").and_then(|v| v.as_array()).unwrap();
    assert!(
        volumes.iter().any(|v| v.get("id").and_then(|i| i.as_u64()) == Some(id)),
        "created volume in list"
    );

    // Get
    let got: serde_json::Value = client
        .cloud()
        .storage()
        .get_volume(id)
        .await
        .expect("get volume");
    assert_eq!(got.get("volume").and_then(|v| v.get("name")).and_then(|n| n.as_str()), Some(name.as_str()));

    // Update
    let update_body = json!({"name": name_updated});
    let _: serde_json::Value = client
        .cloud()
        .storage()
        .update_volume(id, update_body)
        .await
        .expect("update volume");

    // Delete
    let _: serde_json::Value = client
        .cloud()
        .storage()
        .delete_volume(id)
        .await
        .expect("delete volume");
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
    // Minimal valid ED25519 public key for testing
    let public_key = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl test@example.com";

    // Create
    let create_body = json!({"name": name, "public_key": public_key});
    let created = client
        .cloud()
        .create_ssh_key(None, Some(create_body))
        .await
        .expect("create ssh key");
    let key = created.get("ssh_key").and_then(|k| k.get("id"));
    let id = key.and_then(|i| i.as_u64()).expect("ssh key id in response");

    // List and verify
    let list: serde_json::Value = client
        .cloud()
        .list_ssh_keys(None, None)
        .await
        .expect("list ssh keys");
    let keys = list.get("ssh_keys").and_then(|k| k.as_array()).unwrap();
    assert!(
        keys.iter().any(|k| k.get("id").and_then(|i| i.as_u64()) == Some(id)),
        "created ssh key in list"
    );

    // Get
    let got: serde_json::Value = client
        .cloud()
        .get_ssh_key(id, None, None)
        .await
        .expect("get ssh key");
    assert_eq!(got.get("ssh_key").and_then(|k| k.get("name")).and_then(|n| n.as_str()), Some(name.as_str()));

    // Update
    let update_body = json!({"name": name_updated});
    let _: serde_json::Value = client
        .cloud()
        .update_ssh_key(id, None, Some(update_body))
        .await
        .expect("update ssh key");

    // Delete
    let _: serde_json::Value = client
        .cloud()
        .delete_ssh_key(id, None, None)
        .await
        .expect("delete ssh key");
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

    // Create (start_after_create: false to avoid boot delay)
    // Use cpx22 (CPX Gen2) and fsn1 - cpx11 may be deprecated in some locations
    let created = client
        .cloud()
        .servers()
        .create(&CreateServerInput {
            name: name.clone(),
            server_type: "cpx22".to_string(),
            image: "ubuntu-22.04".to_string(),
            location: Some("fsn1".to_string()),
            start_after_create: Some(false),
        })
        .await
        .expect("create server");
    let server_id = created.server.id;

    // Wait for server to leave initializing state (up to ~2 min)
    for _ in 0..24 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let got = client.cloud().servers().get(server_id).await.expect("get server");
        if got.server.status != hetzner::ServerStatus::Initializing {
            break;
        }
    }

    // List and verify
    let servers = client
        .cloud()
        .servers()
        .list(Some(&hetzner::ListServersParams {
            name: Some(name.clone()),
            ..Default::default()
        }))
        .await
        .expect("list servers");
    assert!(
        servers.iter().any(|s| s.id == server_id),
        "created server in list"
    );

    // Get
    let got = client.cloud().servers().get(server_id).await.expect("get server");
    assert_eq!(got.server.name, name);

    // Update
    let update_body = json!({"name": name_updated});
    let _: serde_json::Value = client
        .cloud()
        .servers_api()
        .update(server_id, update_body)
        .await
        .expect("update server");

    // Delete
    let _: serde_json::Value = client
        .cloud()
        .servers_api()
        .delete(server_id)
        .await
        .expect("delete server");
}
