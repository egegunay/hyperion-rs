use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct SharedState {
    pub data: HashMap<String, Arc<Mutex<u64>>>,
}

pub type State = Arc<RwLock<SharedState>>;

pub fn new_state() -> State {
    Arc::new(RwLock::new(SharedState {
        data: HashMap::new(),
    }))
}
