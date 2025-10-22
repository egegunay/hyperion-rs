use crate::state::State;
use std::{convert::Infallible, sync::Arc};

// AI helped with the exceptions, I wanna understand deadlocks and poisoning later on

pub async fn insert(key: String, state: Arc<State>) -> Result<String, Infallible> {
    let mut guard = state.data.write().expect("RwLock poisoned");

    let value = guard.entry(key).or_insert(0);

    *value += 1;

    Ok(value.to_string())
}

pub async fn read(key: String, state: Arc<State>) -> Result<String, Infallible> {
    let guard = state.data.read().expect("RwLock poisoned");

    if let Some(value) = guard.get(&key) {
        Ok(value.to_string())
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

pub async fn update(key: String, new_value: u64, state: Arc<State>) -> Result<String, Infallible> {
    let mut guard = state.data.write().expect("RwLock poisoned");

    if let Some(value) = guard.get_mut(&key) {
        *value = new_value;
        Ok(format!("Key '{}' is now value {}", key, new_value))
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

pub async fn delete(key: String, state: Arc<State>) -> Result<String, Infallible> {
    let mut guard = state.data.write().expect("RwLock poisoned");

    if let Some(value) = guard.remove(&key) {
        Ok(format!("Key '{}' with value {} is deleted", key, value))
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}
