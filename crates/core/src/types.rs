use serde::{Deserialize, Serialize};

pub type Vector = Vec<f32>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointId(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    pub id: PointId,
    pub vector: Vector,
    pub payload: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetricKind {
    Cosine,
    Euclidean,
    Dot,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub vector: Vector,
    pub top_k: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoredPoint {
    pub id: PointId,
    pub score: f32,
    pub payload: serde_json::Value,
}
