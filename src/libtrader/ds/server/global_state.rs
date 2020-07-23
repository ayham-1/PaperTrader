#[allow(unused_imports)]
use crate::ds::server::master_state::MasterState;
#[allow(unused_imports)]
use crate::ds::server::worker_state::WorkerState;

#[derive(Default, PartialEq, Debug)]
pub struct GlobalState {
    #[cfg(feature="master_server")]
    pub state: MasterState,
    #[cfg(feature="worker_server")]
    pub state: WorkerState,
}
