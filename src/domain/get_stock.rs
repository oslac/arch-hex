use super::{model, Error, StockRange, StockReq, StockRes};
use crate::services::db::Database;

pub fn get_stock(req: StockReq, db: &Database) -> Result<StockRes, Error> {
    db.get(model::StockId::new(req.id)).map(|s| s.into())
}

pub fn get_all(db: &Database) -> Result<StockRange, Error> {
    db.get_all()
}
