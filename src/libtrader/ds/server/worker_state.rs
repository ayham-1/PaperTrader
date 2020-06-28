use std::net::Ipv4Addr;

use crate::ds::account::session::SessionID;

#[derive(PartialEq, Debug)]
pub struct WorkerState {
    pub master_server_ip: Ipv4Addr,
    pub sessions: Vec<SessionID>,
}
