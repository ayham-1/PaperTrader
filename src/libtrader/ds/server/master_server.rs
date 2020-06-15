use std::collections::HashMap;
use crate::ds::server::worker_server::WorkerServer;
use crate::ds::account::session::SessionID;

#[derive(Default, PartialEq, Debug)]
pub struct MasterServer {
    pub worker_servers: Vec<WorkerServer>,
    pub active_sessions: Vec<SessionID>,
    pub assets_data: HashMap,
}
