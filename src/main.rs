use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::convert::Infallible;
use warp::Filter;

#[derive(Clone)]
pub struct State {
    pub data: HashMap<String, u64>
}

impl State {
    pub fn new() -> Self {
        Self {
            data: HashMap::default()
        }
    }
}

// Below you can find CRUD methods for API

/// This is inserter.
pub async fn insert(key: String, state: Arc<Mutex<State>>) -> Result<String, Infallible> {
    let mut state = state.lock().unwrap(); // Copied from guides, gotta study

    let current_value = state.data.get(key.as_str());
    
    let new_value = match current_value {
        Some(value) => value + 1,
        None => 1
    };

    println!("Game: {0}, Count: {1}", key, new_value);

    state.data.insert(key, new_value); 

    Ok(new_value.to_string())
}


#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(State::new()));
    

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("insert" / String)
    .and_then({
        let state = state.clone();
        move |p| insert(p, state.clone())
    });

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}