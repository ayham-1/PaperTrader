use either::*;

use std::io::Write;
use rustls;
use rustls::Session;

use crate::network::tls_connection::TlsConnection;
use crate::network::tls_client::TlsClient;

#[cfg(feature="master_server")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, _buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_left(), true);
    let connection = conn.left().unwrap();

    let response = b"HTTP/1.0 200 OK\r\nConnection: close\r\n\r\nHello world from rustls tlsserver\r\n";
    connection.tls_session
        .write_all(response)
        .unwrap();
    connection.tls_session.send_close_notify();
    Ok(())
}

#[cfg(feature="worker_server")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, _buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_left(), true);
    Ok(())
}

#[cfg(feature="client")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_right(), true);
    println!("buffer: {}", String::from_utf8(buf.to_vec()).unwrap());
    Ok(())
}
