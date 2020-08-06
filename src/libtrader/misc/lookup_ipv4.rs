use std::net::{SocketAddr, ToSocketAddrs};

/// Looks up a string hostname and port, returns a SocketAddr represnting that hostname and port.
///
/// Arguments:
/// host - hostname to search.
/// port - port on hostname to search.
///
/// Returns: SocketAddr
pub fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr;
        }
    }

    unreachable!("Cannot lookup address");
}
