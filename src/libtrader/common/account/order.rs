#[derive(PartialEq, Debug)]
pub struct Order {
    pub is_buy: bool,
    pub stock_symbol: String,
    pub stock_price: f64,
    pub stock_amount: isize,
    pub stock_filled: isize,
    pub is_filled: bool,
}
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {}, {})", self.is_buy, self.stock_symbol, self.stock_price, 
               self.stock_amount, self.stock_amount, self.stock_filled, self.is_filled)
    }
}
