mod error;
mod openapi;
mod routes;
mod seq;
mod state;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::path::PathBuf;
use std::sync::Arc;
use tmf_client::config::TmfConfigFile;
use tmf_client::TmfClient;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_path = std::env::var("TMF_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("config.json"));

    let config = TmfConfigFile::from_file(&config_path).unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    // Initialise tracing: console fmt + optional Seq layer
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let seq_layer = config.seq.as_ref().map(|s| {
        let url = std::env::var("SEQ_URL").unwrap_or_else(|_| s.url.clone());
        let key = if s.api_key.is_empty() {
            None
        } else {
            Some(s.api_key.clone())
        };
        seq::SeqLayer::init(url, key)
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .with(seq_layer)
        .init();

    let host = config.portal.host.clone();
    let port = config.portal.port;
    let cors_origin = config.portal.cors_origin.clone();

    let configured = config.configured_apis();
    tracing::info!(
        count = configured.len(),
        path = %config_path.display(),
        "Loaded TMF API endpoints"
    );
    for (api_id, entry) in &configured {
        tracing::info!(api_id = %api_id, name = %entry.name, base_url = %entry.base_url, "  configured");
    }

    let tmf_client = TmfClient::new(config);
    let app_state = web::Data::new(Arc::new(state::AppState::new(tmf_client)));

    let openapi = openapi::ApiDoc::openapi();

    let static_dir = std::env::var("STATIC_DIR")
        .unwrap_or_else(|_| "./static".to_string());
    let serve_static = std::path::Path::new(&static_dir).is_dir();
    if serve_static {
        tracing::info!(path = %static_dir, "Serving frontend static files");
    }

    tracing::info!(host = %host, port = port, "Starting portal backend");

    HttpServer::new(move || {
        let cors = if cors_origin == "*" {
            Cors::permissive()
        } else {
            Cors::default()
                .allowed_origin(&cors_origin)
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
                .allowed_headers(vec!["Content-Type", "Accept", "Authorization"])
                .max_age(3600)
        };

        let app = App::new()
            .wrap(cors)
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .configure(routes::configure);

        if serve_static {
            app.service(
                actix_files::Files::new("/", &static_dir)
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(
                            format!("{}/index.html", static_dir)
                        ).expect("index.html must exist in static dir")
                    ),
            )
        } else {
            app
        }
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
