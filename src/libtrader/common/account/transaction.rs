use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Transaction {
    pub stock_symbol: String,
    pub shares_size: i64,
    pub shares_cost: i64,
    pub is_buy: bool
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.stock_symbol, self.shares_size, self.shares_cost, self.is_buy)
    }
}
