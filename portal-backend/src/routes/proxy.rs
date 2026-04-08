use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tmf_client::query::TmfQueryParams;

use crate::error::AppError;
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/tmf/{api_id}/{resource}",
    tag = "TMF Proxy",
    params(
        ("api_id" = String, Path, description = "TMF API identifier (e.g. tmf620)"),
        ("resource" = String, Path, description = "Resource name (e.g. productOffering)"),
        ("fields" = Option<String>, Query, description = "Comma-separated field names for partial response"),
        ("offset" = Option<u32>, Query, description = "Pagination offset (0-based)"),
        ("limit" = Option<u32>, Query, description = "Maximum items to return"),
        ("sort" = Option<String>, Query, description = "Sort expression (prefix with - for descending)"),
    ),
    responses(
        (status = 200, description = "Resource list"),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn list_resource(
    state: web::Data<Arc<AppState>>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let (api_id, resource) = path.into_inner();
    let client = state.require_api(&api_id)?;
    let params = TmfQueryParams::from_query_string(req.query_string());
    let result = client.list(&resource, &params).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/tmf/{api_id}/{resource}/{id}",
    tag = "TMF Proxy",
    params(
        ("api_id" = String, Path, description = "TMF API identifier"),
        ("resource" = String, Path, description = "Resource name"),
        ("id" = String, Path, description = "Resource identifier"),
    ),
    responses(
        (status = 200, description = "Single resource"),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn get_resource(
    state: web::Data<Arc<AppState>>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, AppError> {
    let (api_id, resource, id) = path.into_inner();
    let client = state.require_api(&api_id)?;
    let result = client.get(&resource, &id).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    post,
    path = "/api/tmf/{api_id}/{resource}",
    tag = "TMF Proxy",
    params(
        ("api_id" = String, Path, description = "TMF API identifier"),
        ("resource" = String, Path, description = "Resource name"),
    ),
    request_body = serde_json::Value,
    responses(
        (status = 201, description = "Resource created"),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn create_resource(
    state: web::Data<Arc<AppState>>,
    path: web::Path<(String, String)>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, AppError> {
    let (api_id, resource) = path.into_inner();
    let client = state.require_api(&api_id)?;
    let result = client.create(&resource, &body).await?;
    Ok(HttpResponse::Created().json(result))
}

#[utoipa::path(
    patch,
    path = "/api/tmf/{api_id}/{resource}/{id}",
    tag = "TMF Proxy",
    params(
        ("api_id" = String, Path, description = "TMF API identifier"),
        ("resource" = String, Path, description = "Resource name"),
        ("id" = String, Path, description = "Resource identifier"),
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Resource patched"),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn patch_resource(
    state: web::Data<Arc<AppState>>,
    path: web::Path<(String, String, String)>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, AppError> {
    let (api_id, resource, id) = path.into_inner();
    let client = state.require_api(&api_id)?;
    let result = client.patch(&resource, &id, &body).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    delete,
    path = "/api/tmf/{api_id}/{resource}/{id}",
    tag = "TMF Proxy",
    params(
        ("api_id" = String, Path, description = "TMF API identifier"),
        ("resource" = String, Path, description = "Resource name"),
        ("id" = String, Path, description = "Resource identifier"),
    ),
    responses(
        (status = 204, description = "Resource deleted"),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn delete_resource(
    state: web::Data<Arc<AppState>>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, AppError> {
    let (api_id, resource, id) = path.into_inner();
    let client = state.require_api(&api_id)?;
    client.delete(&resource, &id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf/{api_id}")
            .route("/{resource}", web::get().to(list_resource))
            .route("/{resource}", web::post().to(create_resource))
            .route("/{resource}/{id}", web::get().to(get_resource))
            .route("/{resource}/{id}", web::patch().to(patch_resource))
            .route("/{resource}/{id}", web::delete().to(delete_resource)),
    );
}
