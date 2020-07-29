use std::net::Ipv4Addr;

#[derive(PartialEq, Debug)]
pub struct WorkerServer {
    pub name: String,
    pub server_ip: Ipv4Addr,
}
impl std::fmt::Display for WorkerServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.server_ip)
    }
}
