use super::model::StockId;
use super::{Error, StockReq};
use crate::services::db::Database;

pub fn delete_stock(req: StockReq, db: &Database) -> Result<(), Error> {
    db.delete(StockId::new(req.id))
}
