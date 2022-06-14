use hexa::config;
use std::sync::Arc;

pub mod server;

pub mod api {
    pub mod health;
    pub mod stonks;
}

fn main() {
    let db = config::init_repo();
    server::serve("localhost:8080", Arc::new(db))
}
