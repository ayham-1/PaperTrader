use crate::ds::account::position::PositionType;

#[derive(PartialEq, Debug)]
pub struct Order {
    pub action_type: PositionType,
    pub stock_symbol: String,
    pub stock_price: f64,
    pub stock_amount: isize,
    pub stock_filled: isize,
    pub is_filled: bool,
}
