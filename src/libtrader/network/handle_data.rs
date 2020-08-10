use either::*;

use crate::network::tls_connection::TlsConnection;
use crate::network::tls_client::TlsClient;

#[cfg(feature="master_server")]
pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, buf: &[u8]) -> Result<(), String> {
    use ring::rand::SecureRandom;
    use ring::{digest, rand};
    use std::io::Write;
    use crate::parser::message_builder::message_builder;
    use crate::ds::message::message::Message;
    use crate::ds::message::message_type::MessageType;
    use crate::ds::message::inst::CommandInst;

    assert_eq!(conn.is_left(), true);
    let connection = conn.left().unwrap();

    let client_response: Message = bincode::deserialize(&buf).unwrap();

    match client_response.instruction {
        _ if client_response.instruction == CommandInst::Register as i64 => {
            let server_response: Message = match message_builder(MessageType::ServerReturn, 1, 0, 0, 0, Vec::new()) {
                Ok(message) => message,
                Err(_err) => panic!("FAILED CREATING MESSAGE")
            };
            connection.tls_session.write(bincode::serialize(&server_response).unwrap().as_slice()).unwrap();
        },
        _ if client_response.instruction == CommandInst::GenHashSalt as i64 => {
            let rng = rand::SystemRandom::new();
            let mut salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
            rng.fill(&mut salt).unwrap();

            let server_response: Message = match message_builder(MessageType::DataTransfer, 
                                                                 CommandInst::GenHashSalt as i64, 1, 0, 1, 
                                                                 salt.to_vec()) {
                Ok(message) => message,
                Err(_) => panic!("PANIK NO SALT")
            };
            connection.tls_session.write(bincode::serialize(&server_response).unwrap().as_slice()).unwrap();
        },
        _ if client_response.instruction == CommandInst::GetEmailSalt as i64 => {
            let rng = rand::SystemRandom::new();
            let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
            rng.fill(&mut salt).unwrap();

            let server_response: Message = match message_builder(MessageType::DataTransfer, 
                                                                 CommandInst::GetEmailSalt as i64, 1, 0, 1, 
                                                                 salt.to_vec()) {
                Ok(message) => message,
                Err(_) => panic!("PANIK NO SALT")
            };
            connection.tls_session.write(bincode::serialize(&server_response).unwrap().as_slice()).unwrap();
        },
        _ if client_response.instruction == CommandInst::GetPasswordSalt as i64 => {
            let rng = rand::SystemRandom::new();
            let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
            rng.fill(&mut salt).unwrap();

            let server_response: Message = match message_builder(MessageType::DataTransfer, 
                                                                 CommandInst::GetPasswordSalt as i64, 1, 0, 1, 
                                                                 salt.to_vec()) {
                Ok(message) => message,
                Err(_) => panic!("PANIK NO SALT")
            };
            connection.tls_session.write(bincode::serialize(&server_response).unwrap().as_slice()).unwrap();
        },
        _ if client_response.instruction == CommandInst::LoginMethod1 as i64 => {
            let server_response: Message = match message_builder(MessageType::ServerReturn, 1, 0, 0, 0, Vec::new()) {
                Ok(message) => message,
                Err(_err) => panic!("FAILED CREATING MESSAGE")
            };
            connection.tls_session.write(bincode::serialize(&server_response).unwrap().as_slice()).unwrap();
        },
        _ => {}
    };
        
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
