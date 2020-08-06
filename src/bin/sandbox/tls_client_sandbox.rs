use std::net::SocketAddr;
use mio;
use mio::net::TcpStream;
use std::io::Write;

use libtrader::network::tls_client::TlsClient;
use libtrader::misc::gen_tls_client_config::gen_tls_client_config;

fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {
    use std::net::ToSocketAddrs;

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr;
        }
    }

    unreachable!("Cannot lookup address");
}

pub fn tls_main() {
    let addr = lookup_ipv4("localhost", 4000);
    let config = gen_tls_client_config();

    let sock = TcpStream::connect(addr).unwrap();
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();
    let mut tlsclient = TlsClient::new(sock, dns_name, config);

    let httpreq = format!("GET / HTTP/1.0\r\nHost: {}\r\nConnection: \
                               close\r\nAccept-Encoding: identity\r\n\r\n",
                               "localhost");
    tlsclient.write_all(httpreq.as_bytes()).unwrap();

    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(32);
    tlsclient.register(poll.registry());

    loop {
        poll.poll(&mut events, None).unwrap();

        for ev in events.iter() {
            tlsclient.ready(&ev);
            tlsclient.reregister(poll.registry());
        }
    }
}
