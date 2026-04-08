use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "TMF Portal API",
        version = "1.0.0",
        description = "Aggregated TMF Open API proxy portal"
    ),
    paths(
        crate::routes::admin::get_api_status,
        crate::routes::admin::health_check,
        crate::routes::proxy::list_resource,
        crate::routes::proxy::get_resource,
        crate::routes::proxy::create_resource,
        crate::routes::proxy::patch_resource,
        crate::routes::proxy::delete_resource,
    ),
    components(schemas(
        crate::routes::admin::ApiStatusEntry,
        crate::routes::admin::ApiStatusResponse,
    )),
    tags(
        (name = "Admin", description = "Portal administration endpoints"),
        (name = "TMF Proxy", description = "Generic TMF API proxy endpoints"),
    )
)]
pub struct ApiDoc;
