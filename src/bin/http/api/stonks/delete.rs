use hexa::domain;
use hexa::ports::DeleteStockCommand;
use hexa::services::db::Repo;

use rouille::Response;
use std::sync::Arc;

pub fn serve(id: usize, db: Arc<dyn Repo>) -> Response {
    let service = domain::DeleteStock { db };
    let req = domain::StockReq { id, symbol: None };

    match service.delete(req) {
        Ok(()) => Response::json(&"OK"),
        Err(domain::Error::NotFound(_)) => Response::json(&"NOT FOUND").with_status_code(404),
        _ => Response::json(&"INTERNAL").with_status_code(500),
    }
}
