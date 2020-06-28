use std::net::Ipv4Addr;

#[derive(PartialEq, Debug)]
pub struct WorkerServer {
    pub name: String,
    pub server_ip: Ipv4Addr,
}
