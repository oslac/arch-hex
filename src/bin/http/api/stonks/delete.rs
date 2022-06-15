use crate::server::SharedDb;
use hexa::domain;
use rouille::Response;

pub fn serve(id: usize, db: SharedDb) -> Response {
    let req = domain::StockReq { id, symbol: None };

    match domain::delete_stock(req, &*db) {
        Ok(()) => Response::json(&"OK"),
        Err(domain::Error::NotFound(_)) => Response::json(&"NOT FOUND").with_status_code(404),
        _ => Response::json(&"INTERNAL").with_status_code(500),
    }
}
