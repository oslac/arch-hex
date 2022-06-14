use crate::domain::model::{Stock, StockId};
use crate::domain::{Error, StockRange};

mod imm_db;
mod lite_db;

pub use imm_db::ImmDb;
pub use lite_db::LiteDb;

pub trait Repo: GetAll + Get + Put + Delete {}

pub trait Get {
    fn get(&self, stock_id: StockId) -> Result<Stock, Error>;
}

pub trait GetAll {
    fn get_all(&self) -> Result<StockRange, Error>;
}

pub trait Put {
    fn put(&self, stock: &Stock) -> Result<(), Error>;
}

pub trait Delete: Send + Sync {
    fn delete(&self, id: StockId) -> Result<(), Error>;
}
