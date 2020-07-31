use postgres_types::{ToSql, FromSql};

#[derive(Default, PartialEq, Debug, ToSql, FromSql)]
pub struct StockVal {
    pub id: i64,
    pub isin: String,
    pub time_since_epoch: i64,
    pub ask_price: i64,
    pub bid_price: i64,
    pub volume: i64
}
impl std::fmt::Display for StockVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {})", self.id, self.isin, self.time_since_epoch, self.ask_price, 
               self.bid_price, self.volume)
    }
}
