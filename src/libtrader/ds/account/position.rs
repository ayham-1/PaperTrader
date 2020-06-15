use chrono::{DateTime, Utc};

#[derive(PartialEq, Debug)]
pub enum PositionType { Sell, Buy }

#[derive(PartialEq, Debug)]
pub struct Position {
    pub action_type: PositionType,
    pub stock_symbol: String,
    pub stock_open_amount: i64,
    pub stock_open_price: i64,
    pub stock_open_cost: i64,
    pub stock_close_amount: i64,
    pub stock_close_price: i64,
    pub stock_close_cost: i64,
    pub open_date: DateTime<Utc>,
    pub close_date: DateTime<Utc>,
    pub is_open: bool,
}
