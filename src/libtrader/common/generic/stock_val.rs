use postgres_types::{ToSql, FromSql};
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, ToSql, FromSql, Serialize, Deserialize)]
pub struct StockVal {
    pub id: i64,
    pub isin: String,
    pub time_epoch: i64,
    pub ask_price: f64,
    pub bid_price: f64,
    pub volume: i64,
}
impl std::fmt::Display for StockVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {}, {})",
            self.id, self.isin, self.time_epoch, self.ask_price, self.bid_price, self.volume
        )
    }
}
