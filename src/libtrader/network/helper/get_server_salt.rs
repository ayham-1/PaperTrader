use ring::digest;
use mio;

use std::io::{Write};

use crate::network::tls_client::TlsClient;
use crate::network::helper::wait_and_read_branched::wait_and_read_branched;
use crate::parser::message_builder::message_builder;
use crate::ds::message::message::Message;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::inst::{CommandInst};

pub fn get_server_salt(tls_client: &mut TlsClient, poll: &mut mio::Poll) -> 
Result<[u8; digest::SHA512_OUTPUT_LEN/2], String> {
    /*
     * request to generate a salt from the server.
     * */
    match message_builder(MessageType::Command, CommandInst::GenHashSalt as i64, 0, 0, 0, vec!()) {
        Ok(message) => {
            tls_client.tls_session.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();

            wait_and_read_branched(tls_client, poll, None, None)?;
            let ret_msg: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
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
