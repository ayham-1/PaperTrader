use chrono::{DateTime, Utc};

#[derive(PartialEq, Debug)]
pub enum PositionType { Sell, Buy }

#[derive(PartialEq, Debug)]
pub struct Position {
    pub action_type: PositionType,
    pub stock_symbol: String,
    pub stock_open_amount: isize,
    pub stock_open_price: f64,
    pub stock_open_cost: f64,
    pub stock_close_amount: isize,
    pub stock_close_price: f64,
    pub stock_close_cost: f64,
    pub open_date: DateTime<Utc>,
    pub close_date: DateTime<Utc>,
    pub is_open: bool,
}
