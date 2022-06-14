use super::{Delete, Get, GetAll, Put, Repo};
use crate::domain::model::{Stock, StockId, Symbol};
use crate::domain::{Error, StockRange};
use std::collections::HashMap;
use std::sync::RwLock;

type Db = RwLock<HashMap<StockId, Stock>>;

#[derive(Default, Debug)]
pub struct ImmDb {
    data: Db,
}

impl ImmDb {
    pub fn new() -> Self {
        let s = Stock::new(StockId::new(1), Symbol::new("AAP".to_string()));
        let data = RwLock::new(HashMap::from([(s.id.clone(), s)]));
        Self { data }
    }
}

impl Repo for ImmDb {}

impl Delete for ImmDb {
    fn delete(&self, id: StockId) -> Result<(), Error> {
        self.data.write().unwrap().remove(&id);
        Ok(())
    }
}

impl Put for ImmDb {
    fn put(&self, stock: &Stock) -> Result<(), Error> {
        self.data.write().unwrap().insert(stock.id.clone(), stock.clone());
        Ok(())
    }
}

impl Get for ImmDb {
    fn get(&self, id: StockId) -> Result<Stock, Error> {
        let res = self.data.read().unwrap().get(&id).cloned();
        res.ok_or_else(|| Error::NotFound(id.id()))
    }
}

impl GetAll for ImmDb {
    fn get_all(&self) -> Result<StockRange, Error> {
        Ok(StockRange {
            stocks: self.data.read().unwrap().iter().map(|(_, v)| v.clone()).collect(),
        })
    }
}
