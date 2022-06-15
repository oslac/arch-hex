use hexa::services::db;
use std::sync::{Arc};

pub mod server;

pub mod api {
    pub mod health;
    pub mod stonks;
}

fn main() {
    let db = Arc::new(db::mk_lite());
    server::serve("localhost:8080", db)
}
