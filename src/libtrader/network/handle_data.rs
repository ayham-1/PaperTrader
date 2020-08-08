use either::*;

use std::io::Write;

use crate::network::tls_connection::TlsConnection;
use crate::network::tls_client::TlsClient;

use crate::parser::message_builder::message_builder;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::inst::CommandInst;

#[cfg(feature="master_server")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, _buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_left(), true);
    let connection = conn.left().unwrap();

    println!("{:?}", _buf);

    match message_builder(MessageType::DataTransfer, CommandInst::GenHashSalt as i64, 1, 0, 1, [3; 32].to_vec()) {
        Ok(message) => {
            connection.tls_session
                .write_all(bincode::serialize(&message).unwrap().as_slice())
                .unwrap();
        },
        _ => {}
    }
    
    Ok(())
}

#[cfg(feature="worker_server")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, _buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_left(), true);
    Ok(())
}

#[cfg(feature="client")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, _buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_right(), true);
    Ok(())
}
