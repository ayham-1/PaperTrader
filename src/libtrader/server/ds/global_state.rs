use std::collections::HashMap;

use crate::common::generic::company::Company;
use crate::common::generic::stock_val::StockVal;

#[derive(PartialEq, Debug)]
pub struct GlobalState {
    pub companies: HashMap<String, Company>,   // symbol, company
    pub stock_vals: HashMap<String, StockVal>, // symbol, stockval
}
impl std::fmt::Display for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?}, {:#?})", self.companies, self.stock_vals)
    }
}
