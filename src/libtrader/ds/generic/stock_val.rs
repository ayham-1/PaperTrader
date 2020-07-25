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
