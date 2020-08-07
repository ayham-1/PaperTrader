use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use std::io::{Write, Read};
use std::convert::TryInto;

use crate::network::tls_client::TlsClient;
use crate::parser::message_builder::message_builder;
use crate::ds::message::message::Message;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::inst::{CommandInst};

pub fn get_server_hash(tls_client: &mut TlsClient) -> Result<[u8; digest::SHA512_OUTPUT_LEN/2], String> {
    /*
     * request to generate a salt from the server.
     * */
    match message_builder(MessageType::Command, CommandInst::GenHashSalt as i64, 0, 0, 0,vec!()) {
        Ok(message) => {
            tls_client.tls_session.write_all(bincode::serialize(&message).unwrap().as_slice()).unwrap();
            let mut returned = Vec::new();
            tls_client.tls_session.read(&mut returned).unwrap();
            let ret_msg: Message = bincode::deserialize(&returned).unwrap();
            assert_eq!(ret_msg.message_type, MessageType::DataTransfer);
            assert_eq!(ret_msg.instruction, CommandInst::GenHashSalt as i64);
            assert_eq!(ret_msg.argument_count, 1);
            assert_eq!(ret_msg.data_message_number, 0);
            assert_eq!(ret_msg.data_message_max, 1);
            assert_eq!(ret_msg.data.len(), digest::SHA512_OUTPUT_LEN/2);
            Ok(*array_ref!(ret_msg.data, 0, digest::SHA512_OUTPUT_LEN/2))
        },
        Err(err) => Err(format!("AUTH_SALT_RETRIEVAL_FAILED: {}", err))
    }
}
