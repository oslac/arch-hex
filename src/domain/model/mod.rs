#[derive(Debug, Clone)]
pub struct Stock {
    pub id: StockId,
    pub symbol: Symbol,
}

#[derive(PartialEq, Clone, Hash, PartialOrd, Ord, Eq, Debug)]
pub struct StockId(usize);

#[derive(Debug, Clone)]
pub struct Symbol(String);

impl Stock {
    pub fn new(id: StockId, symbol: Symbol) -> Self {
        Self { id, symbol }
    }

    pub fn id(&self) -> usize {
        self.id.0
    }

    pub fn symbol(&self) -> &str {
        &self.symbol.0
    }
}

impl StockId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

impl Symbol {
    pub fn new(symbol: String) -> Self {
        Self(symbol)
    }
}

impl From<StockId> for usize {
    fn from(id: StockId) -> Self {
        id.0
    }
}

impl From<Symbol> for String {
    fn from(symbol: Symbol) -> Self {
        symbol.0
    }
}
