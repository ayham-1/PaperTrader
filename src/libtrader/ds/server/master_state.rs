use crate::ds::server::worker_server::WorkerServer;
use crate::ds::account::session::SessionID;

#[derive(Default, PartialEq, Debug)]
pub struct MasterState {
    pub worker_servers: Vec<WorkerServer>,
    pub active_sessions: Vec<SessionID>,
}
