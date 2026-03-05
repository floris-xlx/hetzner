use hetzner::HetznerClient;
use httpmock::prelude::*;
use serde_json::json;

#[tokio::test]
async fn test_cloud_facades_routes() {
    let server = MockServer::start();
    let client = HetznerClient::new("cloud-token").with_cloud_base_url(server.base_url());

    let m_servers = server.mock(|when, then| {
        when.method(GET)
            .path("/servers")
            .header("authorization", "Bearer cloud-token");
        then.status(200)
            .header("content-type", "application/json")
            .body("{}");
    });
    let _ = client.cloud().servers_api().list(None).await.unwrap();
    m_servers.assert();

    let m_domains = server.mock(|when, then| {
        when.method(GET)
            .path("/zones")
            .header("authorization", "Bearer cloud-token");
        then.status(200)
            .header("content-type", "application/json")
            .body("{}");
    });
    let _ = client.cloud().domains().list(None).await.unwrap();
    m_domains.assert();

    let m_networks = server.mock(|when, then| {
        when.method(GET)
            .path("/networks")
            .header("authorization", "Bearer cloud-token");
        then.status(200)
            .header("content-type", "application/json")
            .body("{}");
    });
    let _ = client.cloud().private_networks().list(None).await.unwrap();
    m_networks.assert();

    let m_load_balancers = server.mock(|when, then| {
        when.method(GET)
            .path("/load_balancers")
            .header("authorization", "Bearer cloud-token");
        then.status(200)
            .header("content-type", "application/json")
            .body("{}");
    });
    let _ = client.cloud().load_balancers().list(None).await.unwrap();
    m_load_balancers.assert();

    let m_storage = server.mock(|when, then| {
        when.method(GET)
            .path("/volumes")
            .header("authorization", "Bearer cloud-token");
        then.status(200)
            .header("content-type", "application/json")
            .body("{}");
    });
    let _ = client.cloud().storage().list_volumes(None).await.unwrap();
    m_storage.assert();

    let m_create_server = server.mock(|when, then| {
        when.method(POST)
            .path("/servers")
            .header("authorization", "Bearer cloud-token");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "server": {
                    "id": 1,
                    "name": "srv-1",
                    "status": "running",
                    "created": "2024-01-01T00:00:00Z",
                    "public_net": null,
                    "private_net": [],
                    "labels": null,
                    "server_type": null,
                    "datacenter": null,
                    "image": null,
                    "iso": null,
                    "protection": null,
                    "volumes": [],
                    "load_balancers": [],
                    "placement_group": null,
                    "outgoing_traffic": null,
                    "ingoing_traffic": null,
                    "included_traffic": null,
                    "backup_window": null,
                    "rescue_enabled": null,
                    "locked": null,
                    "primary_disk_size": null
                },
                "action": {
                    "id": 1,
                    "command": "create_server",
                    "status": "running",
                    "started": "2024-01-01T00:00:00Z",
                    "finished": null,
                    "progress": 0,
                    "resources": [],
                    "error": null
                },
                "next_actions": [],
                "root_password": null
            }));
    });

    let _ = client
        .cloud()
        .servers()
        .create(&hetzner::CreateServerInput {
            name: "srv-1".to_string(),
            server_type: "cpx11".to_string(),
            image: "ubuntu-22.04".to_string(),
            location: None,
            start_after_create: None,
        })
        .await
        .unwrap();
    m_create_server.assert();
}
