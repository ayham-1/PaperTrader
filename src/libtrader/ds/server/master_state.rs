use std::collections::HashMap;
use crate::ds::server::worker_server::WorkerServer;
use crate::ds::account::session::SessionID;
use crate::ds::asset::stock::Stock;

#[derive(Default, PartialEq, Debug)]
pub struct MasterState {
    pub worker_servers: Vec<WorkerServer>,
    pub active_sessions: Vec<SessionID>,
    pub assets_data: HashMap<String, Stock>,
}