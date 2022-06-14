use super::model::StockId;
use super::{Error, StockReq};
use crate::ports;
use crate::services::db::{Delete, Repo};
use std::sync::Arc;

pub fn delete(req: StockReq, db: &dyn Delete) -> Result<(), Error> {
    db.delete(StockId::new(req.id))
}

pub struct DeleteStock {
    pub db: Arc<dyn Repo>,
}

impl ports::DeleteStockCommand for DeleteStock {
    fn delete(&self, req: StockReq) -> Result<(), Error> {
        let id = StockId::new(req.id);
        self.db.delete(id)
    }
}
