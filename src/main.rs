mod handlers;
mod state;

use handlers::{delete, insert, read, update}; // CRUD
use state::State;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(State::new()));

    let inserter = warp::path!("insert" / String).and_then({
        let state = state.clone();
        move |p| insert(p, state.clone())
    });

    let reader = warp::path!("read" / String).and_then({
        let state = state.clone();
        move |p| read(p, state.clone())
    });

    let updater = warp::path!("update" / String / u64).and_then({
        let state = state.clone();
        move |p, v| update(p, v, state.clone())
    });

    let deleter = warp::path!("delete" / String).and_then({
        let state = state.clone();
        move |p| delete(p, state.clone())
    });

    let routes = inserter.or(reader).or(updater).or(deleter);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
