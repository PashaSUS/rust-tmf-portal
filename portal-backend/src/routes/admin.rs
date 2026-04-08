use actix_web::{web, HttpResponse};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiStatusEntry {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub base_path: String,
    pub version: String,
    pub versions: Vec<String>,
    pub resources: Vec<String>,
    pub configured: bool,
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    pub resource_schemas: HashMap<String, tmf_client::schema::ResourceSchema>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiStatusResponse {
    pub apis: Vec<ApiStatusEntry>,
}

#[utoipa::path(
    get,
    path = "/api/admin/status",
    tag = "Admin",
    responses(
        (status = 200, description = "API status overview", body = ApiStatusResponse),
    )
)]
pub async fn get_api_status(
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, AppError> {
    let health = state.get_health().await;
    let config = state.tmf_client.config();

    let mut apis: Vec<ApiStatusEntry> = config
        .apis
        .iter()
        .map(|(api_id, entry)| {
            let configured = entry.is_configured();
            let available = health.get(api_id).copied().unwrap_or(false);
            let resource_schemas = tmf_client::schemas::get_api_schemas(api_id);
            ApiStatusEntry {
                id: api_id.clone(),
                name: entry.name.clone(),
                domain: entry.domain.clone(),
                base_path: entry.base_path.clone(),
                version: entry.version.clone(),
                versions: entry.versions.clone(),
                resources: entry.resources.clone(),
                configured,
                available,
                base_url: if configured {
                    Some(entry.base_url.clone())
                } else {
                    None
                },
                resource_schemas,
            }
        })
        .collect();

    apis.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(HttpResponse::Ok().json(ApiStatusResponse { apis }))
}

#[utoipa::path(
    get,
    path = "/api/admin/health",
    tag = "Admin",
    responses(
        (status = 200, description = "Portal health check"),
    )
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/status", web::get().to(get_api_status))
            .route("/health", web::get().to(health_check)),
    );
}
