use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertRequest<TPayload = serde_json::Value> {
    pub id: String,
    pub vector: Vec<f32>,
    #[serde(default)]
    pub payload: TPayload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchRequest {
    pub vector: Vec<f32>,
    pub top_k: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResponse<TPayload = serde_json::Value> {
    pub results: Vec<ScoredPoint<TPayload>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoredPoint<TPayload = serde_json::Value> {
    pub id: String,
    pub score: f32,
    pub payload: TPayload,
}
