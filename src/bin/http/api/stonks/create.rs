use hexa::domain::{self, StockReq};
use hexa::ports::CreateStockCommand;
use hexa::services::db::Repo;

use rouille::input::json_input;
use rouille::{try_or_400, Request, Response};
use std::sync::Arc;

// Request type
#[derive(serde::Deserialize)]
struct Req {
    id: String,
    symbol: String,
}

pub fn serve(req: &Request, db: Arc<dyn Repo>) -> Response {
    let service = domain::CreateStock { db };
    let stock_req = {
        let json: Req = try_or_400!(json_input(req));
        let id: usize = try_or_400!(json.id.parse());
        let symbol = Some(json.symbol);
        StockReq { id, symbol }
    };

    match service.create_stock(stock_req) {
        Ok(()) => Response::json(&"OK"),
        Err(domain::Error::Conflict) => Response::json(&"CONFLICT").with_status_code(409),
        _ => Response::json(&"INTERNAL").with_status_code(500),
    }
}
