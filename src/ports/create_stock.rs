use crate::domain::{self, Error, StockReq};
use crate::services::db::Put;

pub fn create_stock(req: StockReq, db: &dyn Put) -> Result<(), Error> {
    domain::create_stock(req, db)
}

// Alternative implementation of ports.
pub trait CreateStockCommand: Send + Sync {
    fn create_stock(&self, stock_req: StockReq) -> Result<(), Error>;
}
