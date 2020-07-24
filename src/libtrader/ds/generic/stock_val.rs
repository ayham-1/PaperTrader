#[derive(Default, PartialEq, Debug)]
pub struct StockVal {
    pub id: u8,
    pub isin: String,
    pub time_since_epoch: i128,
    pub ask_price: i8,
    pub bid_price: i8,
    pub volume: i8
}
