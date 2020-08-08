use mio;
use mio::net::TcpStream;
use std::io::Write;

use libtrader::network::tls_client::TlsClient;
use libtrader::network::helper::get_server_salt::get_server_salt;
use libtrader::misc::gen_tls_client_config::gen_tls_client_config;
use libtrader::misc::lookup_ipv4::lookup_ipv4;

pub fn tls_main() {
    let addr = lookup_ipv4("0.0.0.0", 4000);
    let config = gen_tls_client_config();

    let sock = TcpStream::connect(addr).unwrap();
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();
    let mut tlsclient = TlsClient::new(sock, dns_name, config);

    
    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(32);
    tlsclient.register(poll.registry());

    loop {
        poll.poll(&mut events, None).unwrap();

        for ev in &events {
            tlsclient.ready(&ev);
            tlsclient.reregister(poll.registry());
            
            if ev.token() == mio::Token(0) && ev.is_writable() {
                match get_server_salt(&mut tlsclient, &mut  poll) {
                    Ok(salt) => println!("{:?}", salt),
                    Err(err) => panic!("panik {}", err),
                }
            }

        }
    } 
}
