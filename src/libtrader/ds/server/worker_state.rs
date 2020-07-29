use std::net::Ipv4Addr;

use crate::ds::account::session::SessionID;

#[derive(PartialEq, Debug)]
pub struct WorkerState {
    pub master_server_ip: Ipv4Addr,
    pub sessions: Vec<SessionID>,
}
impl std::fmt::Display for WorkerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:#?})", self.master_server_ip, self.sessions)
    }
}
