use std::io::Write;
use data_encoding::HEXUPPER;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;
use crate::common::message::inst::{CommandInst, DataTransferInst};

use crate::server::network::tls_connection::TlsConnection;
use crate::server::network::cmd::register::register;
use crate::server::network::cmd::login_normal::login_normal;
use crate::server::network::cmd::retrieve_portfolio::retrieve_portfolio;

pub fn handle_data(connection: &mut TlsConnection, buf: &[u8]) -> 
Result<(), String> {
    /* decode incoming message */
    let client_msg: Message = match bincode::deserialize(&buf) {
        Ok(msg) => msg,
        Err(err) => {
            warn!("HANDLE_DATA_RCVD_INV_MSG: {}", err); 
            connection.closing = true; /* disconnect any unrecognized message senders */
            return Ok(());
        }
    };

    /* handle individual client instructions */
    match client_msg.instruction {
        _ if client_msg.instruction == CommandInst::GenHashSalt as i64 => {
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
            connection.do_tls_write_and_handle_error();
        },
        _ if client_msg.instruction == CommandInst::GetEmailSalt as i64 => {
            use crate::server::db::cmd::get_user_salt::get_user_salt;
            let salt = match get_user_salt(String::from_utf8(client_msg.data).unwrap().as_str(), true, false) {
                Ok(salt) => salt,
                Err(_) => {
                    let msg = match message_builder(MessageType::ServerReturn, 0, 0, 0, 0, Vec::new()) {
                        Ok(message) => message,
                        Err(_) => {error!("HANDLE_DATA_SERVER_COULD_NOT_BUILD_MESSAGE"); return Ok(())},
                    };
                    connection.tls_session.write(&bincode::serialize(&msg).unwrap()).unwrap();
                    connection.do_tls_write_and_handle_error();
                    return Ok(());
                }
            };
            let server_response: Message = match message_builder(MessageType::DataTransfer, 
                                                                 CommandInst::GetEmailSalt as i64, 1, 0, 1, 
                                                                 HEXUPPER.decode(salt.as_bytes()).unwrap()) {
                Ok(message) => message,
                Err(_) => panic!("PANIK NO SALT")
            };
            connection.tls_session.write(bincode::serialize(&server_response).unwrap().as_slice()).unwrap();
            connection.do_tls_write_and_handle_error();
        },
        _ if client_msg.instruction == CommandInst::GetPasswordSalt as i64 => {
            use crate::server::db::cmd::get_user_salt::get_user_salt;
            let salt = match get_user_salt(String::from_utf8(client_msg.data).unwrap().as_str(), false, false) {
                Ok(salt) => salt,
                Err(_) => {
                    let msg = match message_builder(MessageType::ServerReturn, 0, 0, 0, 0, Vec::new()) {
                        Ok(message) => message,
                        Err(_) => {error!("HANDLE_DATA_SERVER_COULD_NOT_BUILD_MESSAGE"); return Ok(())},
                    };
                    connection.tls_session.write(&bincode::serialize(&msg).unwrap()).unwrap();
                    connection.do_tls_write_and_handle_error();
                    return Ok(());
                }
            };
            let server_response: Message = match message_builder(MessageType::DataTransfer, 
                                                                 CommandInst::GetPasswordSalt as i64, 1, 0, 1, 
                                                                 HEXUPPER.decode(salt.as_bytes()).unwrap()) {
                Ok(message) => message,
                Err(_) => panic!("PANIK NO SALT")
            };
            connection.tls_session.write(bincode::serialize(&server_response).unwrap().as_slice()).unwrap();
            connection.do_tls_write_and_handle_error();
        },
        _ if client_msg.instruction == CommandInst::Register as i64 => 
            register(connection, &client_msg),
        _ if client_msg.instruction == CommandInst::LoginMethod1 as i64 => 
            login_normal(connection, &client_msg),
        _ if client_msg.instruction == DataTransferInst::GetUserPortfolio as i64 =>
            retrieve_portfolio(connection, &client_msg),
        _ => {}
    };
        
    Ok(())
}
