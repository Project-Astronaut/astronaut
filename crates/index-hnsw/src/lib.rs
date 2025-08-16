use astronaut_core::{Index, Point, Result, SearchQuery, ScoredPoint};
use std::sync::{Arc, RwLock};

pub struct HnswIndex {
    dim: usize,
    data: Arc<RwLock<Vec<Point>>>,
}

impl HnswIndex {
    pub fn new(dim: usize) -> Self { Self { dim, data: Arc::new(RwLock::new(Vec::new())) } }
}

impl Index for HnswIndex {
    fn name(&self) -> &'static str { "hnsw-placeholder" }
    fn dim(&self) -> usize { self.dim }

    fn insert(&self, point: Point) -> Result<()> {
        self.data.write().unwrap().push(point);
        Ok(())
    }

    fn remove(&self, _id: &str) -> Result<()> { Ok(()) }

    fn search(&self, query: &SearchQuery) -> Result<Vec<ScoredPoint>> {
        let data = self.data.read().unwrap();
        let mut res: Vec<ScoredPoint> = data.iter().map(|p| {
            let score = cosine(&query.vector, &p.vector);
            ScoredPoint { id: p.id.clone(), score, payload: p.payload.clone() }
        }).collect();
        res.sort_by(|a,b| b.score.partial_cmp(&a.score).unwrap());
        res.truncate(query.top_k);
        Ok(res)
    }
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32; let mut na = 0.0f32; let mut nb = 0.0f32;
    for i in 0..a.len() { dot += a[i]*b[i]; na += a[i]*a[i]; nb += b[i]*b[i]; }
    dot / (na.sqrt()*nb.sqrt() + 1e-12)
}
