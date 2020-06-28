use std::next::Ipv4Addr;

#[derive(Default, PartialEq, Debug)]
pub struct WorkerServer {
    pub name: String,
    pub server_ip: Ipv4Addr,
}
