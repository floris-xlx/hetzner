use crate::api::{
    cloud::CloudApi,
    dns::{DnsApi, records::UpdateRecordInput},
};
use crate::error::{ApiError, ApiErrorEnvelope, HetznerError, Result};
use crate::types::{CreatedRecord, Record, RecordEnvelope, Zone};
use reqwest::{header::HeaderMap, Method, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::time::Instant;
use tracing::{debug, error};

const DEFAULT_DNS_BASE_URL: &str = "https://dns.hetzner.com/api/v1";
const DEFAULT_CLOUD_BASE_URL: &str = "https://api.hetzner.cloud/v1";

#[derive(Debug, Clone)]
pub struct HetznerClient {
    pub(crate) http: reqwest::Client,
    pub(crate) auth_api_token: String,
    pub(crate) dns_base_url: String,
    pub(crate) cloud_base_url: String,
}

impl HetznerClient {
    pub fn new(auth_api_token: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            auth_api_token: auth_api_token.into(),
            dns_base_url: DEFAULT_DNS_BASE_URL.to_string(),
            cloud_base_url: DEFAULT_CLOUD_BASE_URL.to_string(),
        }
    }

    pub fn with_dns_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.dns_base_url = base_url.into();
        self
    }

    #[deprecated(note = "Use with_dns_base_url(...) or with_cloud_base_url(...) instead.")]
    pub fn with_base_url(self, base_url: impl Into<String>) -> Self {
        self.with_dns_base_url(base_url)
    }

    pub fn with_cloud_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.cloud_base_url = base_url.into();
        self
    }

    pub fn dns(&self) -> DnsApi<'_> {
        DnsApi { client: self }
    }

    pub fn cloud(&self) -> CloudApi<'_> {
        CloudApi { client: self }
    }

    pub(crate) async fn request_dns<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<T> {
        self.request_to_base(
            &self.dns_base_url,
            "Auth-API-Token",
            "",
            method,
            path,
            Option::<&Vec<(String, String)>>::None,
            body,
        )
        .await
    }

    pub(crate) async fn request_dns_unit(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<()> {
        self.request_unit_to_base(
            &self.dns_base_url,
            "Auth-API-Token",
            "",
            method,
            path,
            Option::<&Vec<(String, String)>>::None,
            body,
        )
        .await
    }

    pub(crate) async fn request_cloud<T: DeserializeOwned, Q: Serialize>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<Value>,
    ) -> Result<T> {
        self.request_to_base(
            &self.cloud_base_url,
            "Authorization",
            "Bearer ",
            method,
            path,
            query,
            body,
        )
        .await
    }

    async fn request_to_base<T: DeserializeOwned, Q: Serialize>(
        &self,
        base_url: &str,
        auth_header: &str,
        auth_prefix: &str,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<Value>,
    ) -> Result<T> {
        let url = format!("{}/{}", base_url.trim_end_matches('/'), path);
        let method_for_log = method.clone();
        let start = Instant::now();
        let mut req = self
            .http
            .request(method, &url)
            .header(auth_header, format!("{auth_prefix}{}", self.auth_api_token));

        if let Some(params) = query {
            req = req.query(params);
        }

        if let Some(payload) = body {
            req = req.json(&payload);
        }

        let response = req.send().await?;
        let status = response.status();
        let request_id = response
            .headers()
            .get("X-Request-Id")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_owned());
        let rate_limit = rate_limit_snapshot(response.headers());
        let body_bytes = response.bytes().await?;

        if status.is_success() {
            // 204 No Content returns empty body; treat as JSON null for parsing
            let body_to_parse = if body_bytes.is_empty() {
                b"null" as &[u8]
            } else {
                &body_bytes
            };
            match serde_json::from_slice::<T>(body_to_parse) {
                Ok(parsed) => {
                    debug!(
                        method = %method_for_log,
                        %url,
                        status = %status,
                        request_id = request_id.as_deref().unwrap_or(""),
                        rate_limit_limit = rate_limit.limit,
                        rate_limit_remaining = rate_limit.remaining,
                        rate_limit_reset = rate_limit.reset,
                        elapsed_ms = start.elapsed().as_millis(),
                        "hetzner request succeeded"
                    );
                    return Ok(parsed);
                }
                Err(err) => {
                    error!(
                        method = %method_for_log,
                        %url,
                        status = %status,
                        request_id = request_id.as_deref().unwrap_or(""),
                        rate_limit_limit = rate_limit.limit,
                        rate_limit_remaining = rate_limit.remaining,
                        rate_limit_reset = rate_limit.reset,
                        parse_error = %err,
                        body_snippet = %truncate_for_log(&String::from_utf8_lossy(&body_bytes), 1024),
                        elapsed_ms = start.elapsed().as_millis(),
                        "hetzner request parse failed"
                    );
                    return Err(err.into());
                }
            }
        }

        let body_text = String::from_utf8_lossy(&body_bytes).to_string();
        let api_error = parse_api_error(status, body_text.clone());
        error!(
            method = %method_for_log,
            %url,
            status = %status,
            code = %api_error.code,
            request_id = request_id.as_deref().unwrap_or(""),
            rate_limit_limit = rate_limit.limit,
            rate_limit_remaining = rate_limit.remaining,
            rate_limit_reset = rate_limit.reset,
            retry_after = rate_limit.retry_after,
            elapsed_ms = start.elapsed().as_millis(),
            body_snippet = %truncate_for_log(&body_text, 1024),
            "hetzner request failed"
        );
        Err(HetznerError::Api(api_error))
    }

    async fn request_unit_to_base<Q: Serialize>(
        &self,
        base_url: &str,
        auth_header: &str,
        auth_prefix: &str,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<Value>,
    ) -> Result<()> {
        let url = format!("{}/{}", base_url.trim_end_matches('/'), path);
        let method_for_log = method.clone();
        let start = Instant::now();
        let mut req = self
            .http
            .request(method, &url)
            .header(auth_header, format!("{auth_prefix}{}", self.auth_api_token));

        if let Some(params) = query {
            req = req.query(params);
        }

        if let Some(payload) = body {
            req = req.json(&payload);
        }

        let response = req.send().await?;
        let status = response.status();
        let request_id = response
            .headers()
            .get("X-Request-Id")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_owned());
        let rate_limit = rate_limit_snapshot(response.headers());
        let body_bytes = response.bytes().await?;

        if status.is_success() {
            debug!(
                method = %method_for_log,
                %url,
                status = %status,
                request_id = request_id.as_deref().unwrap_or(""),
                rate_limit_limit = rate_limit.limit,
                rate_limit_remaining = rate_limit.remaining,
                rate_limit_reset = rate_limit.reset,
                elapsed_ms = start.elapsed().as_millis(),
                "hetzner request succeeded"
            );
            return Ok(());
        }

        let body_text = String::from_utf8_lossy(&body_bytes).to_string();
        let api_error = parse_api_error(status, body_text.clone());
        error!(
            method = %method_for_log,
            %url,
            status = %status,
            code = %api_error.code,
            request_id = request_id.as_deref().unwrap_or(""),
            rate_limit_limit = rate_limit.limit,
            rate_limit_remaining = rate_limit.remaining,
            rate_limit_reset = rate_limit.reset,
            retry_after = rate_limit.retry_after,
            elapsed_ms = start.elapsed().as_millis(),
            body_snippet = %truncate_for_log(&body_text, 1024),
            "hetzner request failed"
        );
        Err(HetznerError::Api(api_error))
    }

    #[deprecated(
        note = "Legacy DNS methods on HetznerClient are deprecated. Use client.dns().list_zones()."
    )]
    pub async fn get_all_zones(&self) -> Result<Vec<Zone>> {
        self.dns().list_zones().await
    }

    #[deprecated(
        note = "Legacy DNS methods on HetznerClient are deprecated. Use client.dns().records(zone_id).list()."
    )]
    pub async fn get_all_records(&self, zone_id: &str) -> Result<Vec<Record>> {
        self.dns().records(zone_id).list().await
    }

    #[deprecated(
        note = "Legacy DNS methods on HetznerClient are deprecated. Use client.dns().records(zone_id).create(...)."
    )]
    pub async fn create_record(
        &self,
        value: &str,
        ttl: u64,
        type_: &str,
        name: &str,
        zone_id: &str,
    ) -> Result<CreatedRecord> {
        self.dns()
            .records(zone_id)
            .create(name, type_, value, ttl)
            .await
    }

    #[deprecated(
        note = "Legacy DNS methods on HetznerClient are deprecated. Use client.dns().record(record_id)."
    )]
    pub async fn get_record(&self, record_id: &str) -> Result<RecordEnvelope> {
        self.dns().record(record_id).get().await
    }

    #[deprecated(
        note = "Legacy DNS methods on HetznerClient are deprecated. Use client.dns().record(record_id).update(...)."
    )]
    pub async fn update_record(
        &self,
        record_id: &str,
        zone_id: &str,
        type_: &str,
        name: &str,
        value: &str,
        ttl: u64,
    ) -> Result<RecordEnvelope> {
        self.dns()
            .record(record_id)
            .update(UpdateRecordInput {
                zone_id: zone_id.to_string(),
                record_type: type_.to_string(),
                name: name.to_string(),
                value: value.to_string(),
                ttl,
            })
            .await
    }

    #[deprecated(
        note = "Legacy DNS methods on HetznerClient are deprecated. Use client.dns().record(record_id).delete()."
    )]
    pub async fn delete_record(&self, record_id: &str) -> Result<()> {
        self.dns().record(record_id).delete().await
    }
}

fn parse_api_error(status: StatusCode, body_text: String) -> ApiError {
    let parsed_error = serde_json::from_str::<ApiErrorEnvelope>(&body_text);

    match parsed_error {
        Ok(envelope) => ApiError {
            status,
            code: envelope.error.code,
            message: envelope.error.message,
            details: envelope.error.details,
        },
        Err(_) => ApiError {
            status,
            code: status_code_to_default_code(status).to_string(),
            message: body_text,
            details: None,
        },
    }
}

fn truncate_for_log(body: &str, max_len: usize) -> String {
    let mut chars = body.chars();
    let prefix: String = chars.by_ref().take(max_len).collect();
    if chars.next().is_some() {
        format!("{prefix}... (truncated)")
    } else {
        prefix
    }
}

#[derive(Debug, Clone, Copy)]
struct RateLimitSnapshot {
    limit: Option<u64>,
    remaining: Option<u64>,
    reset: Option<u64>,
    retry_after: Option<u64>,
}

fn rate_limit_snapshot(headers: &HeaderMap) -> RateLimitSnapshot {
    RateLimitSnapshot {
        limit: header_u64(headers, "RateLimit-Limit"),
        remaining: header_u64(headers, "RateLimit-Remaining"),
        reset: header_u64(headers, "RateLimit-Reset"),
        retry_after: header_u64(headers, "Retry-After"),
    }
}

fn header_u64(headers: &HeaderMap, name: &str) -> Option<u64> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
}

fn status_code_to_default_code(status: StatusCode) -> &'static str {
    match status {
        StatusCode::BAD_REQUEST => "json_error",
        StatusCode::UNAUTHORIZED => "unauthorized",
        StatusCode::FORBIDDEN => "forbidden",
        StatusCode::NOT_FOUND => "not_found",
        StatusCode::METHOD_NOT_ALLOWED => "method_not_allowed",
        StatusCode::CONFLICT => "conflict",
        StatusCode::GONE => "deprecated_api_endpoint",
        StatusCode::PRECONDITION_FAILED => "resource_unavailable",
        StatusCode::UNPROCESSABLE_ENTITY => "invalid_input",
        StatusCode::LOCKED => "locked",
        StatusCode::TOO_MANY_REQUESTS => "rate_limit_exceeded",
        StatusCode::INTERNAL_SERVER_ERROR => "server_error",
        StatusCode::SERVICE_UNAVAILABLE => "unavailable",
        StatusCode::GATEWAY_TIMEOUT => "timeout",
        _ => "unknown_error",
    }
}
