use crate::state::State;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};

// AI helped with the exceptions, I wanna understand deadlocks and poisoning later on

pub async fn insert(key: String, state: State) -> Result<String, Infallible> {
    let value_arc = {
        if let Some(v) = state.read().expect("RwLock poisoned").data.get(&key) {
            v.clone()
        } else {
            let mut write_guard = state.write().expect("RwLock poisoned");
            write_guard
                .data
                .entry(key.clone())
                .or_insert_with(|| Arc::new(Mutex::new(0)))
                .clone()
        }
    };

    let mut value = value_arc.lock().expect("Mutex poisoned");
    *value += 1;

    Ok(value.to_string())
}

pub async fn read(key: String, state: State) -> Result<String, Infallible> {
    let value_arc = state
        .read()
        .expect("RwLock poisoned")
        .data
        .get(&key)
        .cloned();

    if let Some(value_arc) = value_arc {
        let value = value_arc.lock().expect("Mutex poisoned");
        Ok(value.to_string())
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

pub async fn update(key: String, new_value: u64, state: State) -> Result<String, Infallible> {
    if let Some(value_arc) = state
        .read()
        .expect("RwLock poisoned")
        .data
        .get(&key)
        .cloned()
    {
        let mut value = value_arc.lock().expect("Mutex poisoned");
        *value = new_value;
        Ok(format!("Key '{}' is now value {}", key, new_value))
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

pub async fn delete(key: String, state: State) -> Result<String, Infallible> {
    let value_arc = state.write().expect("RwLock poisoned").data.remove(&key);

    if let Some(value_arc) = value_arc {
        let value = value_arc.lock().expect("Mutex poisoned"); // One more lock for the love of the game
        Ok(format!("Key '{}' with value {} is deleted", key, *value))
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}
