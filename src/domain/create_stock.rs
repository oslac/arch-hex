use super::model::{Stock, StockId, Symbol};
use super::{Error, StockReq};
use crate::ports;
use crate::services::db::{Put, Repo};
use std::sync::Arc;

pub fn create_stock(req: StockReq, db: &dyn Put) -> Result<(), Error> {
    let symbol = Symbol::new(req.symbol.unwrap());
    let id = StockId::new(req.id);
    let stock = Stock::new(id, symbol);
    db.put(&stock)
}

pub struct CreateStock {
    pub db: Arc<dyn Repo>,
}

impl ports::CreateStockCommand for CreateStock {
    fn create_stock(&self, req: StockReq) -> Result<(), Error> {
        let symbol = Symbol::new(req.symbol.unwrap());
        let id = StockId::new(req.id);
        let stock = Stock::new(id, symbol);
        self.db.put(&stock)
    }
}
