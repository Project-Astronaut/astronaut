use axum::{routing::{get, post}, Router, Json};
use axum::http::StatusCode;
use astronaut_core::{InMemoryStorage, Point, PointId, Storage};
use astronaut_api_types::{InsertRequest, SearchRequest, SearchResponse, ScoredPoint};
use std::sync::Arc;
use tracing::info;
use serde_json::json;

#[derive(Clone)]
struct AppState {
    storage: InMemoryStorage,
    dim: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let state = AppState { storage: InMemoryStorage::new(), dim: 1536 };

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/insert", post(insert))
        .route("/v1/search", post(search))
        .with_state(Arc::new(state));

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({"status":"ok","service":"astral"}))
}

async fn insert(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<InsertRequest>,
) -> Result<StatusCode, StatusCode> {
    if payload.vector.len() != state.dim { return Err(StatusCode::BAD_REQUEST); }
    let point = Point { id: PointId(payload.id), vector: payload.vector, payload: payload.payload };
    state.storage.put(point).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::CREATED)
}

async fn search(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, StatusCode> {
    if req.vector.len() != state.dim { return Err(StatusCode::BAD_REQUEST); }

    // Placeholder linear scan over storage using snapshot
    let mut results: Vec<ScoredPoint> = Vec::new();
    let points = state.storage.snapshot();
    for p in points.iter() {
        let score = cosine(&req.vector, &p.vector);
        results.push(ScoredPoint { id: p.id.0.clone(), score, payload: p.payload.clone() });
    }
    results.sort_by(|a,b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(req.top_k);
    Ok(Json(SearchResponse { results }))
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for i in 0..a.len() {
        dot += a[i]*b[i];
        na += a[i]*a[i];
        nb += b[i]*b[i];
    }
    dot / (na.sqrt()*nb.sqrt() + 1e-12)
}
