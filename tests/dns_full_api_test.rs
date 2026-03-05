use hetzner::HetznerClient;
use hetzner::api::dns::records::UpdateRecordInput;
use httpmock::prelude::*;
use serde_json::json;

fn zone_json() -> serde_json::Value {
    json!({
        "created": "2024-01-01T00:00:00Z",
        "id": "zone-1",
        "is_secondary_dns": false,
        "legacy_dns_host": "",
        "legacy_ns": [],
        "modified": "2024-01-01T00:00:00Z",
        "name": "example.com",
        "ns": ["ns1.example.com"],
        "owner": "owner-1",
        "paused": false,
        "permission": "read_write",
        "project": "project-1",
        "records_count": 1,
        "registrar": "none",
        "status": "verified",
        "ttl": 3600,
        "txt_verification": {
            "name": "_acme-challenge",
            "token": "token"
        },
        "verified": "verified",
        "zone_type": {
            "description": "primary",
            "id": "zone-type-1",
            "name": "primary",
            "prices": null
        }
    })
}

fn record_json() -> serde_json::Value {
    json!({
        "id": "record-1",
        "name": "www",
        "ttl": 3600,
        "type": "A",
        "value": "1.2.3.4",
        "zone_id": "zone-1",
        "created": "2024-01-01T00:00:00Z",
        "modified": "2024-01-01T00:00:00Z"
    })
}

#[tokio::test]
async fn test_dns_api_full_surface() {
    let server = MockServer::start();
    let client = HetznerClient::new("dns-token").with_dns_base_url(server.base_url());

    let zones_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/zones")
            .header("auth-api-token", "dns-token");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"zones": [zone_json()], "meta": null}));
    });

    let zones = client.dns().list_zones().await.unwrap();
    assert_eq!(zones.len(), 1);
    zones_mock.assert();

    let list_records_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/records")
            .query_param("zone_id", "zone-1")
            .header("auth-api-token", "dns-token");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"records": [record_json()], "meta": null}));
    });

    let records = client.dns().records("zone-1").list().await.unwrap();
    assert_eq!(records.len(), 1);
    list_records_mock.assert();

    let create_record_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/records")
            .header("auth-api-token", "dns-token");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"record": record_json()}));
    });

    let created = client
        .dns()
        .records("zone-1")
        .create("www", "A", "1.2.3.4", 3600)
        .await
        .unwrap();
    assert_eq!(created.record.id, "record-1");
    create_record_mock.assert();

    let get_record_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/records/record-1")
            .header("auth-api-token", "dns-token");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"record": record_json()}));
    });

    let got = client.dns().record("record-1").get().await.unwrap();
    assert_eq!(got.record.name, "www");
    get_record_mock.assert();

    let update_record_mock = server.mock(|when, then| {
        when.method(PUT)
            .path("/records/record-1")
            .header("auth-api-token", "dns-token");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"record": record_json()}));
    });

    let updated = client
        .dns()
        .record("record-1")
        .update(UpdateRecordInput {
            zone_id: "zone-1".to_string(),
            record_type: "A".to_string(),
            name: "www".to_string(),
            value: "1.2.3.4".to_string(),
            ttl: 3600,
        })
        .await
        .unwrap();
    assert_eq!(updated.record.id, "record-1");
    update_record_mock.assert();

    let delete_record_mock = server.mock(|when, then| {
        when.method(DELETE)
            .path("/records/record-1")
            .header("auth-api-token", "dns-token");
        then.status(200)
            .header("content-type", "application/json")
            .body("{}");
    });

    client.dns().record("record-1").delete().await.unwrap();
    delete_record_mock.assert();
}
