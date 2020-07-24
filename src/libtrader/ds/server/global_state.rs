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

    pub companies: HashMap<String, Company>,
    pub stock_vals: HashMap<String, StockVal>,
}
