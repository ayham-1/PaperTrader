use std::net;
use mio;
use mio::net::TcpListener;

use libtrader::network::tls_server::TlsServer;
use libtrader::misc::gen_tls_server_config::gen_tls_server_config;

pub fn tls_main() {
    let addr: net::SocketAddr = "0.0.0.0:4000".parse().unwrap();

    let config = gen_tls_server_config("certs/test_tls.crt", "certs/test_tls.key", None);

    let mut listener = TcpListener::bind(addr).expect("cannot listen on port");
    let mut poll = mio::Poll::new().unwrap();

    poll.registry().register(&mut listener, mio::Token(0), mio::Interest::READABLE).unwrap();

    let mut tlsserv = TlsServer::new(listener, config);

    let mut events = mio::Events::with_capacity(256);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                mio::Token(0) => {
                    tlsserv.accept(poll.registry()).expect("error accepting socket");
                },
                _ => tlsserv.conn_event(poll.registry(), &event)
            }
        }
    }
}
