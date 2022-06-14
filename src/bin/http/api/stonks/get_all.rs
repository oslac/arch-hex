use hexa::domain;
use hexa::ports::GetAllQuery;
use hexa::services::db::Repo;

use rouille::Response;
use std::sync::Arc;

pub fn serve(db: Arc<dyn Repo>) -> Response {
    let service = domain::GetAllStocks { db };

    match service.get_all() {
        Ok(stocks) => {
            let mut data: Vec<Stock> = vec![];
            for s in stocks.stocks {
                let stonk = Stock::new(s.id(), s.symbol());
                data.push(stonk);
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
