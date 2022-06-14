use crate::domain::{self, Error, StockReq};
use crate::services::db::Delete;

pub fn delete(req: StockReq, db: &dyn Delete) -> Result<(), Error> {
    domain::delete(req, db)
}

// Alternative implementation of ports.
pub trait DeleteStockCommand: Send + Sync {
    fn delete(&self, stock_req: StockReq) -> Result<(), Error>;
}
