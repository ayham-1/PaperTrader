use mio;
use mio::net::TcpStream;
use std::io::Write;

use libtrader::network::tls_client::TlsClient;
use libtrader::misc::gen_tls_client_config::gen_tls_client_config;
use libtrader::misc::lookup_ipv4::lookup_ipv4;

use libtrader::account::acc_auth::acc_auth_client;

pub fn tls_main() {
    let addr = lookup_ipv4("localhost", 4000);
    let config = gen_tls_client_config();

    let sock = TcpStream::connect(addr).unwrap();
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();
    let mut tlsclient = TlsClient::new(sock, dns_name, config);

    /* experimentation area */
    //let httpreq = format!("GET / HTTP/1.0\r\nHost: {}\r\nConnection: \
    //                           close\r\nAccept-Encoding: identity\r\n\r\n",
    //                           "localhost");

    //tlsclient.write_all(httpreq.as_bytes()).unwrap();
    /* end of  experimentation area */

    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(32);
    tlsclient.register(poll.registry());

    loop {
        poll.poll(&mut events, None).unwrap();

        for ev in &events {
            tlsclient.ready(&ev);
            tlsclient.reregister(poll.registry());
            if ev.token() == mio::Token(0) && ev.is_writable() {
                match acc_auth_client(&mut tlsclient, "test", "test", "test") {
                    Ok(()) => {}
                    Err(err) => panic!("PANIK"),
                }

            }
        }
    }
}
