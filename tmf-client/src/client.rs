use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::time::{Duration, Instant};

use crate::error::{Result, TmfError};
use crate::query::TmfQueryParams;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub data: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TmfApiClient {
    http: Client,
    base_url: String,
}

impl TmfApiClient {
    pub fn new(base_url: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn format_headers(headers: &reqwest::header::HeaderMap) -> String {
        headers
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.to_str().unwrap_or("?")))
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn truncate(s: &str, max: usize) -> String {
        if s.len() <= max {
            s.to_string()
        } else {
            format!("{}…", &s[..max])
        }
    }

    async fn send_logged(
        &self,
        method: &str,
        url: &str,
        builder: reqwest::RequestBuilder,
        body_preview: Option<&str>,
    ) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let start = Instant::now();

        tracing::info!(
            method = method,
            url = url,
            request_body = body_preview.unwrap_or(""),
            "-> TMF outgoing request"
        );

        let resp = builder.send().await?;
        let elapsed = start.elapsed();
        let status = resp.status().as_u16();
        let headers = Self::format_headers(resp.headers());

        if resp.status().is_success() {
            tracing::info!(
                method = method,
                url = url,
                status = status,
                duration_ms = elapsed.as_millis() as u64,
                response_headers = %headers,
                "<- TMF response"
            );
        } else {
            tracing::warn!(
                method = method,
                url = url,
                status = status,
                duration_ms = elapsed.as_millis() as u64,
                response_headers = %headers,
                "<- TMF error response"
            );
        }

        Ok(resp)
    }

    pub async fn list(&self, path: &str, params: &TmfQueryParams) -> Result<ListResponse> {
        let url = self.build_url(path);
        let pairs = params.to_pairs();
        let builder = self.http.get(&url).query(&pairs);

        let resp = self.send_logged("GET", &url, builder, None).await?;

        let total_count = resp
            .headers()
            .get("X-Total-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!(status = status, error_body = %body, "TMF API error body");
            return Err(TmfError::Api { status, body });
        }

        let data = resp
            .json()
            .await
            .map_err(|e| TmfError::Deserialize(e.to_string()))?;
        Ok(ListResponse { data, total_count })
    }

    pub async fn get(&self, path: &str, id: &str) -> Result<Value> {
        let url = format!("{}/{}", self.build_url(path), id);
        let builder = self.http.get(&url);

        let resp = self.send_logged("GET", &url, builder, None).await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!(status = status, error_body = %body, "TMF API error body");
            return Err(TmfError::Api { status, body });
        }

        resp.json()
            .await
            .map_err(|e| TmfError::Deserialize(e.to_string()))
    }

    pub async fn create(&self, path: &str, body: &Value) -> Result<Value> {
        let url = self.build_url(path);
        let body_str = serde_json::to_string(body).unwrap_or_default();
        let preview = Self::truncate(&body_str, 500);
        let builder = self.http.post(&url).json(body);

        let resp = self
            .send_logged("POST", &url, builder, Some(&preview))
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!(status = status, error_body = %body, "TMF API error body");
            return Err(TmfError::Api { status, body });
        }

        resp.json()
            .await
            .map_err(|e| TmfError::Deserialize(e.to_string()))
    }

    pub async fn patch(&self, path: &str, id: &str, body: &Value) -> Result<Value> {
        let url = format!("{}/{}", self.build_url(path), id);
        let body_str = serde_json::to_string(body).unwrap_or_default();
        let preview = Self::truncate(&body_str, 500);
        let builder = self.http.patch(&url).json(body);

        let resp = self
            .send_logged("PATCH", &url, builder, Some(&preview))
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!(status = status, error_body = %body, "TMF API error body");
            return Err(TmfError::Api { status, body });
        }

        resp.json()
            .await
            .map_err(|e| TmfError::Deserialize(e.to_string()))
    }

    pub async fn delete(&self, path: &str, id: &str) -> Result<()> {
        let url = format!("{}/{}", self.build_url(path), id);
        let builder = self.http.delete(&url);

        let resp = self.send_logged("DELETE", &url, builder, None).await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!(status = status, error_body = %body, "TMF API error body");
            return Err(TmfError::Api { status, body });
        }

        Ok(())
    }

    pub async fn health_check(&self) -> bool {
        self.http
            .get(&self.base_url)
            .timeout(Duration::from_secs(3))
            .send()
            .await
            .is_ok()
    }
}
