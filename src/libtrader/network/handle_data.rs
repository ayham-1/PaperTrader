use either::*;

use std::io::Write;
use rustls;
use rustls::Session;

use crate::network::tls_connection::TlsConnection;
use crate::network::tls_client::TlsClient;

#[cfg(feature="master_server")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_left(), true);
    let connection = conn.left().unwrap();

    println!("{:?}", buf);

    connection.tls_session
        .write_all(buf)
        .unwrap();
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

    println!("buffer: {:?}", buf);
    Ok(())
}
