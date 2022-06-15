use crate::domain::model::{Stock, StockId};
use crate::domain::{Error, StockRange};
mod queries;
mod row;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use row::StockRow;
use rusqlite::params;

use super::{Database, DatabaseImpl};
use queries::{
    CREATE_TABLE_STOCK, DELETE_STOCK_WITH_ID, INSERT_STOCK, SELECT_ALL, SELECT_STOCK_WITH_ID,
};

pub fn mk_lite() -> Database {
    Database(Box::new(LiteDb::default()))
}

#[derive(Debug)]
struct LiteDb {
    pool: Pool<SqliteConnectionManager>,
}

impl DatabaseImpl for LiteDb {
    fn delete(&self, id: StockId) -> Result<(), Error> {
        let conn = self.pool.get().unwrap();
        let res = conn.execute(DELETE_STOCK_WITH_ID, [id.id()]);

        match res {
            // NOT FOUND:
            Ok(0) => Err(Error::NotFound(id.id())),
            // FOUND:
            Ok(1) => Ok(()),
            // SOMETHING HAPPENED:
            _ => Err(Error::Unknown),
        }
    }

    fn get_all(&self) -> Result<StockRange, Error> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare(SELECT_ALL).unwrap();
        let iter = stmt
            .query_map([], |r| {
                let id = r.get(0).unwrap();
                let symbol = r.get(1).unwrap();
                Ok(StockRow { id, symbol })
            })
            .unwrap();

        let mut stonks = StockRange { stocks: vec![] };
        for x in iter {
            stonks.stocks.push(x.unwrap().into())
        }

        Ok(stonks)
    }

    fn create(&self, stock: &Stock) -> Result<(), Error> {
        let conn = &self.pool.get().unwrap();
        let res = conn.execute(INSERT_STOCK, params![stock.id(), stock.symbol()]);

        match res {
            Ok(_) => Ok(()),
            _ => Err(Error::Conflict),
        }
    }

    fn get(&self, id: StockId) -> Result<Stock, Error> {
        let conn = &self.pool.get().unwrap();
        let res = conn.query_row(SELECT_STOCK_WITH_ID, [id.id()], |row| {
            let id = row.get(0).unwrap();
            let symbol = row.get(1).unwrap();
            Ok(StockRow { id, symbol })
        });

        match res {
            Ok(stonk) => Ok(stonk.into()),
            Err(_) => Err(Error::NotFound(id.id())),
        }
    }
}

impl Default for LiteDb {
    fn default() -> Self {
        let manager = SqliteConnectionManager::file("stock.db");
        let pool = r2d2::Pool::new(manager).unwrap();
        pool.get().unwrap().execute(CREATE_TABLE_STOCK, []).unwrap();
        Self { pool }
    }
}
