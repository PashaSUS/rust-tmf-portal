pub mod client;
pub mod config;
pub mod error;
pub mod query;
pub mod schema;
pub mod schemas;

use std::collections::HashMap;

use client::TmfApiClient;
use config::TmfConfigFile;

#[derive(Debug, Clone)]
pub struct TmfClient {
    clients: HashMap<String, TmfApiClient>,
    config: TmfConfigFile,
}

impl TmfClient {
    pub fn new(config: TmfConfigFile) -> Self {
        let mut clients = HashMap::new();
        for (api_id, entry) in &config.apis {
            if entry.is_configured() {
                clients.insert(api_id.clone(), TmfApiClient::new(&entry.base_url));
            }
        }
        Self { clients, config }
    }

    pub fn get(&self, api_id: &str) -> Option<&TmfApiClient> {
        self.clients.get(api_id)
    }

    pub fn config(&self) -> &TmfConfigFile {
        &self.config
    }

    pub fn available_apis(&self) -> Vec<String> {
        self.clients.keys().cloned().collect()
    }

    pub async fn check_health(&self) -> HashMap<String, bool> {
        let mut handles = Vec::new();
        for (api_id, client) in &self.clients {
            let api_id = api_id.clone();
            let client = client.clone();
            handles.push(tokio::spawn(async move {
                (api_id, client.health_check().await)
            }));
        }
        let mut results = HashMap::new();
        for handle in handles {
            if let Ok((api_id, healthy)) = handle.await {
                results.insert(api_id, healthy);
            }
        }
        results
    }
}
