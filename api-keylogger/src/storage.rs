use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Storage {
    inner: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, data: serde_json::Value) {
        let mut store = self.inner.lock().unwrap();
        store.push(data);
    }

    pub fn get_all(&self) -> Vec<serde_json::Value> {
        let store = self.inner.lock().unwrap();
        store.clone()
    }
}
