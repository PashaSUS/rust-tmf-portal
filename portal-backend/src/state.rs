use tmf_client::TmfClient;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct AppState {
    pub tmf_client: TmfClient,
    health_cache: Mutex<Option<(Instant, HashMap<String, bool>)>>,
}

impl AppState {
    pub fn new(tmf_client: TmfClient) -> Self {
        Self {
            tmf_client,
            health_cache: Mutex::new(None),
        }
    }

    pub fn require_api(
        &self,
        api_id: &str,
    ) -> Result<&tmf_client::client::TmfApiClient, crate::error::AppError> {
        self.tmf_client
            .get(api_id)
            .ok_or_else(|| crate::error::AppError::ServiceUnavailable(api_id.to_string()))
    }

    pub async fn get_health(&self) -> HashMap<String, bool> {
        const CACHE_TTL: Duration = Duration::from_secs(30);

        if let Ok(cache) = self.health_cache.lock() {
            if let Some((timestamp, ref data)) = *cache {
                if timestamp.elapsed() < CACHE_TTL {
                    return data.clone();
                }
            }
        }

        let health = self.tmf_client.check_health().await;

        if let Ok(mut cache) = self.health_cache.lock() {
            *cache = Some((Instant::now(), health.clone()));
        }

        health
    }
}
