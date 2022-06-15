use crate::server::SharedDb;
use hexa::domain;
use rouille::Response;

pub fn serve(id: usize, db: SharedDb) -> Response {
    let req = domain::StockReq { id, symbol: None };

    match domain::get_stock(req, &*db) {
        Ok(stonk) => Response::json(&(stonk.id, stonk.symbol)),
        Err(domain::Error::NotFound(id)) => {
            Response::json(&format!("STOCK {id} NOT FOUND")).with_status_code(404)
        }
        _ => Response::json(&"INTERNAL").with_status_code(500),
    }
}
