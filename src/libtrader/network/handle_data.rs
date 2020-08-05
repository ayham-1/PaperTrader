use std::io::Write;

use rustls;
use rustls::Session;

use crate::network::tls_connection::TlsConnection;

#[cfg(feature="master_server")]
pub fn handle_data(connection: &mut TlsConnection, _buf: &[u8]) -> Result<(), String> {
    let response = b"HTTP/1.0 200 OK\r\nConnection: close\r\n\r\nHello world from rustls tlsserver\r\n";
    connection.tls_session
        .write_all(response)
        .unwrap();
    connection.tls_session.send_close_notify();
    Ok(())
}

#[cfg(feature="worker_server")]
pub fn handle_data(_buf: &[u8]) -> Result<(), String> {
    Ok(())
}

#[cfg(feature="client")]
pub fn handle_data(_buf: &[u8]) -> Result<(), String> {
    Ok(())
}
