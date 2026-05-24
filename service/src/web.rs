use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::{Json, Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::Deserialize;

use crate::actor::AppRuntime;
use crate::sync::SyncTrigger;

#[derive(RustEmbed)]
#[folder = "assets"]
struct FrontendAssets;

#[derive(Debug, Deserialize)]
pub struct ConfigForm {
    pub toml: String,
}

type ApiResult = std::result::Result<StatusCode, (StatusCode, String)>;

pub fn build_router(app: Arc<AppRuntime>) -> Router {
    Router::new()
        .route("/api/state", get(api_state))
        .route("/api/config", get(api_config).post(api_update_config_json))
        .route("/api/sync", post(api_sync))
        .route("/api/reload", post(api_reload))
        .route("/api/save", post(api_save))
        .route("/api/stop", post(api_stop))
        .route("/", get(index))
        .route("/{*path}", get(serve_spa_asset))
        .with_state(app)
}

async fn api_state(State(app): State<Arc<AppRuntime>>) -> Response {
    match serde_json::to_vec(&*app.snapshot()) {
        Ok(bytes) => ([(header::CONTENT_TYPE, "application/json")], bytes).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn api_config(State(app): State<Arc<AppRuntime>>) -> Response {
    match serde_json::to_vec(&app.snapshot().config) {
        Ok(bytes) => ([(header::CONTENT_TYPE, "application/json")], bytes).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn api_sync(State(app): State<Arc<AppRuntime>>) -> ApiResult {
    app.clone().request(SyncTrigger::Manual).await;
    Ok(StatusCode::NO_CONTENT)
}

async fn api_reload(State(app): State<Arc<AppRuntime>>) -> ApiResult {
    if let Err(err) = app.reload_config().await {
        app.log(format!("E: reload failed: {err}")).await;
        app.set_status("Reload failed").await;
        return Err((StatusCode::BAD_REQUEST, err.to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn api_save(State(app): State<Arc<AppRuntime>>) -> ApiResult {
    if let Err(err) = app.save_config().await {
        app.log(format!("E: save failed: {err}")).await;
        app.set_status("Save failed").await;
        return Err((StatusCode::BAD_REQUEST, err.to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn api_update_config_json(
    State(app): State<Arc<AppRuntime>>,
    Json(form): Json<ConfigForm>,
) -> ApiResult {
    if let Err(err) = app.update_config_from_toml(&form.toml).await {
        app.log(format!("E: config update failed: {err}")).await;
        app.set_status("Config update failed").await;
        return Err((StatusCode::BAD_REQUEST, err.to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn api_stop(State(app): State<Arc<AppRuntime>>) -> ApiResult {
    app.stop();
    Ok(StatusCode::NO_CONTENT)
}

async fn index() -> Response {
    serve_embed_file("index.html", true)
}

async fn serve_spa_asset(Path(path): Path<String>) -> Response {
    let normalized = path.trim_start_matches('/');
    if let Some(response) = serve_embed_file_if_exists(normalized) {
        return response;
    }

    if should_fallback_to_index(normalized) {
        return serve_embed_file("index.html", true);
    }

    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

fn should_fallback_to_index(path: &str) -> bool {
    !path.contains('.')
}

fn serve_embed_file_if_exists(path: &str) -> Option<Response> {
    FrontendAssets::get(path).map(|asset| build_response(path, asset.data.into_owned(), false))
}

fn serve_embed_file(path: &str, no_store: bool) -> Response {
    match FrontendAssets::get(path) {
        Some(asset) => build_response(path, asset.data.into_owned(), no_store),
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("embedded asset not found: {path}"),
        )
            .into_response(),
    }
}

fn build_response(path: &str, bytes: Vec<u8>, no_store: bool) -> Response {
    let mime = from_path(path).first_or_octet_stream();
    let builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .header(
            header::CACHE_CONTROL,
            if no_store {
                "no-store, max-age=0"
            } else {
                "public, max-age=31536000, immutable"
            },
        );

    builder.body(Body::from(bytes)).unwrap_or_else(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to build embedded response: {err}"),
        )
            .into_response()
    })
}
