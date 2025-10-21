use crate::state::State;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};

pub async fn insert(key: String, state: Arc<Mutex<State>>) -> Result<String, Infallible> {
    let mut state = state.lock().unwrap(); // Copied from guides, gotta study

    let current_value = state.data.get(key.as_str());

    let new_value = match current_value {
        Some(value) => value + 1,
        None => 1,
    };

    state.data.insert(key, new_value); // Possibly can use and_modify?

    Ok(new_value.to_string())
}

pub async fn read(key: String, state: Arc<Mutex<State>>) -> Result<String, Infallible> {
    let state = state.lock().unwrap();

    match state.data.get(&key) {
        Some(value) => Ok(value.to_string()),
        None => Ok(format!("Key '{}' not found", key)),
    }
}

pub async fn update(
    key: String,
    value: u64,
    state: Arc<Mutex<State>>,
) -> Result<String, Infallible> {
    let mut state = state.lock().unwrap();

    let entry = state.data.contains_key(&key);

    if entry {
        state
            .data
            .entry(key.to_string()) // I originally did not do key.to_string, and that broke the line below. I do not understand consuming.
            .and_modify(|val| *val = value);
        Ok(format!("Key '{0}' is now value {1}", key, value))
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

pub async fn delete(key: String, state: Arc<Mutex<State>>) -> Result<String, Infallible> {
    let mut state = state.lock().unwrap();

    let value = state.data.remove(&key);

    match value {
        Some(val) => Ok(format!("Key {0} with value {1} is deleted", key, val)),
        None => Ok(format!("Key {} not found", key)),
    }
}
