use crate::domain::model::{Stock, StockId};
use crate::domain::{Error, StockRange};

mod imm_db;
mod lite_db;
pub use imm_db::mk_imm;
pub use lite_db::mk_lite;

pub struct Database(Box<dyn DatabaseImpl>);

impl Database {
    pub fn create(&self, stock: Stock) -> Result<(), Error> {
        self.0.create(&stock)
    }
    pub fn get(&self, stock_id: StockId) -> Result<Stock, Error> {
        self.0.get(stock_id)
    }
    pub fn get_all(&self) -> Result<StockRange, Error> {
        self.0.get_all()
    }
    pub fn delete(&self, id: StockId) -> Result<(), Error> {
        self.0.delete(id)
    }
}

pub trait DatabaseImpl: Send + Sync {
    fn get(&self, stock_id: StockId) -> Result<Stock, Error>;
    fn get_all(&self) -> Result<StockRange, Error>;
    fn create(&self, stock: &Stock) -> Result<(), Error>;
    fn delete(&self, id: StockId) -> Result<(), Error>;
}
