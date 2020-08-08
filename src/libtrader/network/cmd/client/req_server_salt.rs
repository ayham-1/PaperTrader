use ring::{digest};
use std::io::Write;

use crate::network::tls_client::TlsClient;

use crate::parser::message_builder::message_builder;
use crate::ds::message::message::Message;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::inst::CommandInst;

use crate::network::cmd::generic::wait_and_read_branched::wait_and_read_branched;

pub fn req_server_salt(tls_client: &mut TlsClient, poll: &mut mio::Poll, username: &str, salt_type: i64) -> 
Result<[u8; digest::SHA512_OUTPUT_LEN], String> {
    /* enforce salt_type to be either email or password */
    assert_eq!(salt_type >= CommandInst::GetEmailSalt as i64, true);
    assert_eq!(salt_type <= CommandInst::GetPasswordSalt as i64, true);

    /* generate message to send */
    match message_builder(MessageType::Command, salt_type, 1, 0, 0, 
                          username.as_bytes().to_vec()) {
        Ok(message) => {
            tls_client.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();
            wait_and_read_branched(tls_client, poll, None, None)?;
            let ret_msg: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
            match ret_msg.message_type {
                MessageType::Command => {
                    Err("REQ_SERVER_SALT_INVALID_SERVER_RETURN".to_string())
                },
                MessageType::DataTransfer => {
                    if ret_msg.data.len() != digest::SHA512_OUTPUT_LEN {
                        Err("REQ_SERVER_SALT_INVALID_SERVER_RETURN_SIZE".to_string())
                    } else if ret_msg.instruction == salt_type {
                        Ok(*array_ref!(ret_msg.data, 0, digest::SHA512_OUTPUT_LEN))
                    } else {
                        Err("REQ_SERVER_SALT_INVALID_SERVER_INSTRUCTION_RETURN".to_string())
                    }
                },
                MessageType::ServerReturn => {
                    match ret_msg.instruction {
                        0 => Err("REQ_SERVER_SALT_REJECTED".to_string()),
                        _ => Err("REQ_SERVER_SALT_INVALID_SERVER_RETURN".to_string()),
                    }
                }
            }
        },
        Err(_) => Err("REQ_SERVER_SALT_FAILED".to_string())
    }
}
