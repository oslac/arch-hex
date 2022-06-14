use crate::domain::model::{Stock, StockId, Symbol};

pub struct StockRow {
    pub id: usize,
    pub symbol: String,
}

impl From<StockRow> for Stock {
    fn from(row: StockRow) -> Self {
        Self::new(StockId::new(row.id), Symbol::new(row.symbol))
    }
}
