use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Default)]
pub struct State {
    pub data: RwLock<HashMap<String, u64>>,
}
