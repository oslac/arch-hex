use hexa::domain;

use crate::server::SharedDb;
use rouille::Response;

pub fn serve(db: SharedDb) -> Response {
    match domain::get_all(&*db) {
        Ok(stocks) => {
            let mut data: Vec<Stock> = vec![];
            for s in stocks.stocks {
                data.push(Stock::new(s.id(), s.symbol()));
            }
            Response::json(&data)
        }
        Err(_) => Response::json(&"INTERNAL").with_status_code(500),
    }
}

#[derive(serde::Serialize)]
struct Stock {
    id: usize,
    symbol: String,
}

impl Stock {
    fn new(id: usize, symbol: &str) -> Stock {
        Self { id, symbol: symbol.to_string() }
    }
}
