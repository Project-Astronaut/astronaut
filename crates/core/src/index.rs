use crate::{Point, Result, SearchQuery, ScoredPoint};

pub trait Index: Send + Sync {
    fn name(&self) -> &'static str;
    fn dim(&self) -> usize;

    fn insert(&self, point: Point) -> Result<()>;
    fn upsert(&self, point: Point) -> Result<()> { self.insert(point) }
    fn remove(&self, id: &str) -> Result<()>;

    fn search(&self, query: &SearchQuery) -> Result<Vec<ScoredPoint>>;
}
