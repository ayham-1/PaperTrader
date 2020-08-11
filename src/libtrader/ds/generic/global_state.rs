use std::collections::HashMap;

use crate::ds::generic::company::Company;
use crate::ds::generic::stock_val::StockVal;

use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(PartialEq, Debug)]
pub struct GlobalState {
    pub db_connect_str: String,

    pub companies: HashMap<String, Company>, // symbol, company
    pub stock_vals: HashMap<String, StockVal>, // symbol, stockval
}
impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            db_connect_str: String::default(),
            companies: HashMap::default(),
            stock_vals: HashMap::default(),
        }
    }
}
impl std::fmt::Display for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:#?}, {:#?})", self.db_connect_str, self.companies, self.stock_vals)
    }
}

lazy_static! {
    pub static ref GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(GlobalState::default());
}
