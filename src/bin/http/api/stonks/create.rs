use hexa::domain::{self, StockReq};

use crate::server::SharedDb;
use rouille::input::json_input;
use rouille::{try_or_400, Request, Response};

// Request type
#[derive(serde::Deserialize)]
struct Req {
    id: String,
    symbol: String,
}

pub fn serve(req: &Request, db: SharedDb) -> Response {
    let stock_req = {
        let json: Req = try_or_400!(json_input(req));
        let id: usize = try_or_400!(json.id.parse());
        let symbol = Some(json.symbol);
        StockReq { id, symbol }
    };

    match domain::create_stock(stock_req, &*db) {
        Ok(()) => Response::json(&"OK"),
        Err(domain::Error::Conflict) => Response::json(&"CONFLICT").with_status_code(409),
        _ => Response::json(&"INTERNAL").with_status_code(500),
    }
}
