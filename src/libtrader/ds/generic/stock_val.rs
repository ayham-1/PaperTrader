use postgres_types::{ToSql, FromSql};

#[derive(Default, PartialEq, Debug, ToSql, FromSql)]
pub struct StockVal {
    pub id: i8,
    pub isin: String,
    pub time_since_epoch: i32,
    pub ask_price: i8,
    pub bid_price: i8,
    pub volume: i8
}
