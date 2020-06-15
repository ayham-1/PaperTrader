use chrono::{DateTime, Utc};

#[derive(PartialEq, Debug)]
pub struct StockVal {
    pub open_val: i64,
    pub close_val: i64,
    pub low_val: i64,
    pub high_val: i64,
    pub avg_val: i64,
    pub vol: i64,
    pub open_date: DateTime<Utc>,
    pub close_date: DateTime<Utc>,
}

#[derive(PartialEq, Debug)]
pub struct StockDayVal {
    pub open_val: i64,
    pub close_val: i64,
    pub low_val: i64,
    pub high_val: i64,
    pub vol: i64,
    pub open_date: DateTime<Utc>,
    pub close_date: DateTime<Utc>,
    pub stock_vals: Vec<StockVal>,
}

#[derive(PartialEq, Debug)]
pub struct StockWeekVal {
    pub open_val: i64,
    pub close_val: i64,
    pub low_val: i64,
    pub high_val: i64,
    pub vol: i64,
    pub open_date: DateTime<Utc>,
    pub close_date: DateTime<Utc>,
    pub stock_vals: [StockDayVal; 5],
}

#[derive(PartialEq, Debug)]
pub struct StockMonthVal {
    pub open_val: i64,
    pub close_val: i64,
    pub low_val: i64,
    pub high_val: i64,
    pub vol: i64,
    pub open_date: DateTime<Utc>,
    pub close_date: DateTime<Utc>,
    pub week_vals: [StockWeekVal; 4],
}

#[derive(PartialEq, Debug)]
pub struct StockYearVal {
    pub open_val: i64,
    pub close_val: i64,
    pub low_val: i64,
    pub high_val: i64,
    pub vol: i64,
    pub open_date: DateTime<Utc>,
    pub close_date: DateTime<Utc>,
    pub month_vals: [StockMonthVal; 12],
}

#[derive(Default, PartialEq, Debug)]
pub struct Stock {
    pub symbol: String,
    pub latest_price: i64,
    pub val: Vec<StockYearVal>
}
