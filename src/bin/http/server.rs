use crate::api::*;

use hexa::services::db::Database;
use std::sync::Arc;

pub type SharedDb = Arc<Database>;

pub fn serve(url: &str, db: SharedDb) {
    println!("\nLISTENING @ {url}");

    rouille::start_server(url, move |req| {
        rouille::router!(req,
            (GET) (/stonk/) => {
                stonks::get_all::serve(db.clone())
            },
            (POST) (/stonk/) => {
                stonks::create::serve(req,db.clone())
            },
            (GET) (/stonk/{id: usize}) => {
                stonks::get::serve(id, db.clone())
            },
            (DELETE) (/stonk/{id: usize}) => {
                stonks::delete::serve(id,db.clone())
            },

            (GET) (/health) => {
                health::serve()
             },
            _ => {
                rouille::Response::json(&"NOT FOUND").with_status_code(404)
            }
        )
    });
}
