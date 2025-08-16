use crate::{Point, PointId, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait Storage: Send + Sync {
    fn put(&self, point: Point) -> Result<()>;
    fn get(&self, id: &str) -> Result<Option<Point>>;
    fn delete(&self, id: &str) -> Result<()>;
}

#[derive(Clone, Default)]
pub struct InMemoryStorage {
    inner: Arc<RwLock<HashMap<String, Point>>>,
}

impl InMemoryStorage {
    pub fn new() -> Self { Self::default() }

    // Returns a snapshot clone of all points for naive scans
    pub fn snapshot(&self) -> Vec<Point> {
        let g = self.inner.read().unwrap();
        g.values().cloned().collect()
    }
}

impl Storage for InMemoryStorage {
    fn put(&self, point: Point) -> Result<()> {
        let mut g = self.inner.write().unwrap();
        g.insert(point.id.0.clone(), point);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Option<Point>> {
        let g = self.inner.read().unwrap();
        Ok(g.get(id).cloned())
    }

    fn delete(&self, id: &str) -> Result<()> {
        let mut g = self.inner.write().unwrap();
        g.remove(id);
        Ok(())
    }
}
