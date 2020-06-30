use postgres_types::{ToSql, FromSql};

use chrono::{DateTime, Utc};

#[derive(PartialEq, Debug, ToSql, FromSql)]
pub enum PositionType { Sell, Buy }

#[derive(PartialEq, Debug, ToSql, FromSql)]
pub struct Position {
    pub action_type: PositionType,
    pub stock_symbol: String,
    pub stock_open_amount: i64,
    pub stock_open_price: f64,
    pub stock_open_cost: f64,
    pub stock_close_amount: i64,
    pub stock_close_price: f64,
    pub stock_close_cost: f64,
    //pub open_date: DateTime<Utc>,
    //pub close_date: DateTime<Utc>,
    pub is_open: bool,
}
