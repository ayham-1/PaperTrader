use serde::{Serialize, Deserialize};

enum_from_primitive! {
#[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub enum PositionType { Sell = 0, Buy = 1 }
}
impl std::fmt::Display for PositionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {PositionType::Sell=>"SELL", PositionType::Buy=>"BUY"})
        }
    }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Position {
    pub action_type: PositionType,
    pub stock_symbol: String,
    pub stock_open_amount: i64,
    pub stock_open_price: f64,
    pub stock_open_cost: f64,
    pub stock_close_amount: i64,
    pub stock_close_price: f64,
    pub stock_close_cost: f64,
    pub open_epoch: i64,
    pub close_epoch: i64,
    pub is_open: bool,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", self.action_type, self.stock_symbol, 
               self.stock_open_amount, self.stock_open_price, self.stock_open_cost, self.stock_close_amount, 
               self.stock_close_price, self.stock_close_cost, self.open_epoch, self.close_epoch, self.is_open)
    }
}
