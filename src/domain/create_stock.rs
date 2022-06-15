use super::model::{Stock, StockId, Symbol};
use super::{Error, StockReq};
use crate::services::db::Database;

pub fn create_stock(req: StockReq, db: &Database) -> Result<(), Error> {
    let symbol = Symbol::new(req.symbol.unwrap());
    let id = StockId::new(req.id);
    let stock = Stock::new(id, symbol);
    db.create(stock)
}
