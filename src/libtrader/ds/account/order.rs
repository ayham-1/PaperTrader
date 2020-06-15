use crate::ds::account::position::PositionType;

#[derive(PartialEq, Debug)]
pub struct Order {
    pub action_type: PositionType,
    pub stock_symbol: String,
    pub stock_price: i64,
    pub stock_amount: i64,
    pub stock_filled: i64,
    pub is_filled: bool,
}
