use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    pub data: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}
