use hexa::domain;
use hexa::ports::GetStockQuery;
use hexa::services::db::Repo;

use rouille::Response;
use std::sync::Arc;

pub fn serve(id: usize, db: Arc<dyn Repo>) -> Response {
    let service = domain::GetStock { db };

    match service.get_stock(domain::StockReq { id, symbol: None }) {
        Ok(stonk) => Response::json(&(stonk.id, stonk.symbol)),
        Err(domain::Error::NotFound(id)) => {
            Response::json(&format!("STOCK {id} NOT FOUND")).with_status_code(404)
        }
        _ => Response::json(&"INTERNAL").with_status_code(500),
    }
}
