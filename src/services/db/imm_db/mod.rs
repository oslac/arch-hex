use super::{Database, DatabaseImpl};
use crate::domain::model::{Stock, StockId, Symbol};
use crate::domain::{Error, StockRange};
use std::collections::HashMap;
use std::sync::RwLock;

type Db = RwLock<HashMap<StockId, Stock>>;

pub fn mk_imm() -> Database {
    Database(Box::new(ImmDb::new()))
}

#[derive(Default, Debug)]
struct ImmDb {
    data: Db,
}

impl ImmDb {
    fn new() -> Self {
        let s = Stock::new(StockId::new(1), Symbol::new("AAP".to_string()));
        let data = RwLock::new(HashMap::from([(s.id.clone(), s)]));
        Self { data }
    }
}

impl DatabaseImpl for ImmDb {
    fn delete(&self, id: StockId) -> Result<(), Error> {
        self.data.write().unwrap().remove(&id);
        Ok(())
    }

    fn create(&self, stock: &Stock) -> Result<(), Error> {
        self.data.write().unwrap().insert(stock.id.clone(), stock.clone());
        Ok(())
    }

    fn get(&self, id: StockId) -> Result<Stock, Error> {
        let res = self.data.read().unwrap().get(&id).cloned();
        res.ok_or_else(|| Error::NotFound(id.id()))
    }

    fn get_all(&self) -> Result<StockRange, Error> {
        Ok(StockRange {
            stocks: self.data.read().unwrap().iter().map(|(_, v)| v.clone()).collect(),
        })
    }
}
