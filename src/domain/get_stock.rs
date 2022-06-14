use super::{model, Error, StockRange, StockReq, StockRes};
use crate::ports;
use crate::services::db::{Get, GetAll, Repo};
use std::sync::Arc;

pub fn get_stock(req: StockReq, db: &dyn Get) -> Result<StockRes, Error> {
    db.get(model::StockId::new(req.id)).map(|s| s.into())
}

pub fn get_all(db: &dyn GetAll) -> Result<StockRange, Error> {
    db.get_all()
}

pub struct GetStock {
    pub db: Arc<dyn Repo>,
}

pub struct GetAllStocks {
    pub db: Arc<dyn Repo>,
}

impl ports::GetStockQuery for GetStock {
    fn get_stock(&self, req: StockReq) -> Result<StockRes, Error> {
        match self.db.get(model::StockId::new(req.id)) {
            Ok(stonk) => Ok(stonk.into()),
            Err(_) => Err(Error::NotFound(req.id)),
        }
    }
}

impl ports::GetAllQuery for GetAllStocks {
    fn get_all(&self) -> Result<StockRange, Error> {
        self.db.get_all()
    }
}
