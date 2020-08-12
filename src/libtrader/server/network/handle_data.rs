use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;
use crate::common::message::inst::CommandInst;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::network::cmd::register::register;

pub fn handle_data(connection: &mut TlsConnection, buf: &[u8]) -> 
Result<(), String> {
    /* decode incoming message */
    let client_response: Message = match bincode::deserialize(&buf) {
        Ok(msg) => msg,
        Err(err) => {
            warn!("HANDLE_DATA_RCVD_INV_MSG: {}", err); 
            connection.closing = true; /* disconnect any unrecognized message senders */
            return Ok(());
        }
    };

    /* handle individual client instructions */
    match client_response.instruction {
        _ if client_response.instruction == CommandInst::GenHashSalt as i64 => {
            use ring::rand::SecureRandom;
            use ring::{digest, rand};
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
        _ if client_response.instruction == CommandInst::Register as i64 => 
            register(connection, &client_response),
        _ => {}
    };
        
    Ok(())
}
