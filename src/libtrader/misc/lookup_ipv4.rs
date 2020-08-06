use std::net::{SocketAddr, ToSocketAddrs};

pub fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr;
        }
    }

    unreachable!("Cannot lookup address");
}
