pub(super) const CREATE_TABLE_STOCK: &str = "CREATE TABLE IF NOT EXISTS stock (
    id     INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE
);";

pub(super) const SELECT_STOCK_WITH_ID: &str = "SELECT id, symbol FROM stock WHERE id = ?";
pub(super) const SELECT_ALL: &str = "SELECT id, symbol FROM stock";
pub(super) const INSERT_STOCK: &str = "INSERT INTO stock (id, symbol) VALUES (?, ?)";
pub(super) const DELETE_STOCK_WITH_ID: &str = "DELETE FROM stock WHERE id = ?";
