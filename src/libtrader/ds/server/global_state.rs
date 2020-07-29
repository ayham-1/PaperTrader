use std::collections::HashMap;

#[allow(unused_imports)]
use crate::ds::server::master_state::MasterState;
#[allow(unused_imports)]
use crate::ds::server::worker_state::WorkerState;

use crate::ds::generic::company::Company;
use crate::ds::generic::stock_val::StockVal;

#[derive(Default, PartialEq, Debug)]
pub struct GlobalState {
    #[cfg(feature="master_server")]
    pub state: MasterState,
    #[cfg(feature="worker_server")]
    pub state: WorkerState,

    pub db_connect_str: String,

    pub companies: HashMap<String, Company>, // symbol, company
    pub stock_vals: HashMap<String, StockVal>, // symbol, stockval
}
impl std::fmt::Display for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?}, {}, {:#?}, {:#?})", self.state, self.db_connect_str, self.companies, self.stock_vals)
    }
}
