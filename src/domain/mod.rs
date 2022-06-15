// Types in this file are optional indirection.
//
// Business logic in terms of domain types, separated from external
// infrastructure by an interface.
//
// The 4 use cases contain a function implementation of ports.
mod create_stock;
mod delete_stock;
mod get_stock;

pub use create_stock::create_stock;
pub use delete_stock::delete_stock;
pub use get_stock::{get_all, get_stock};

use self::model::Stock;
use std::fmt::Display;

// Turning this module private should only prompt errors inside `services/db`
pub mod model;

#[derive(Debug)]
pub struct StockReq {
    pub id: usize,
    // A quick and dirty:
    pub symbol: Option<String>,
}

#[derive(Debug)]
pub struct StockRes {
    pub id: usize,
    pub symbol: String,
}

#[derive(Default, Debug)]
pub struct StockRange {
    pub stocks: Vec<Stock>,
}

impl From<model::Stock> for StockRes {
    fn from(stock: model::Stock) -> Self {
        Self { id: stock.id(), symbol: stock.symbol().to_string() }
    }
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    Conflict,
    NotFound(usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Unknown => "INTERNAL".fmt(f),
            Error::Conflict => "CONFLICT".fmt(f),
            Error::NotFound(id) => write!(f, "STOCK {} NOT FOUND", id),
        }
    }
}
