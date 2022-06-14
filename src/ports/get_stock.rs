use crate::domain::StockRange;
use crate::domain::{self, Error};
use crate::domain::{StockReq, StockRes};
use crate::services::db::{Get, GetAll};

pub fn get_stock(req: StockReq, db: &dyn Get) -> Result<StockRes, Error> {
    domain::get_stock(req, db)
}

pub fn get_all(db: &dyn GetAll) -> Result<StockRange, Error> {
    domain::get_all(db)
}

pub trait GetAllQuery: Send + Sync {
    fn get_all(&self) -> Result<StockRange, Error>;
}

pub trait GetStockQuery: Send + Sync {
    fn get_stock(&self, stock_req: StockReq) -> Result<StockRes, Error>;
}
