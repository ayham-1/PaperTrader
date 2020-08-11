use ring::digest;
use mio;

use std::io::{Write};

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::inst::{CommandInst};
use crate::common::message::message_builder::message_builder;

use crate::client::network::tls_client::TlsClient;
use crate::client::network::cmd::wait_and_read_branched::wait_and_read_branched;

/// Issues a command to the connected TLS server to obtain a salt.
///
/// All salts returned are of size ```digest::SHA512_OUTPUT_LEN/2``` or 32 bytes.
///
/// Arguments:
/// tls_client - The TLS connection to use for the salt.
/// poll - The mio::Poll used to handle branched control of the TLS client.
///
/// Returns: a [u8; 32] on success, and a string on error containing the reason of failure.
///
/// Example:
/// ```rust
///     let server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = match get_server_salt(tls_client, poll) {
///         Ok(salt) => salt,
///         Err(err) => panic!("could not retrieve server salt; err: {}", errj)
///     };
/// ```
pub fn get_server_salt(tls_client: &mut TlsClient, poll: &mut mio::Poll) -> 
Result<[u8; digest::SHA512_OUTPUT_LEN/2], String> {
    /*
     * request to generate a salt from the server.
     * */
    match message_builder(MessageType::Command, CommandInst::GenHashSalt as i64, 0, 0, 0, vec!()) {
        Ok(message) => {
            tls_client.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();

            wait_and_read_branched(tls_client, poll, None, None)?;
            let ret_msg: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
            assert_eq!(ret_msg.msgtype, MessageType::DataTransfer);
            assert_eq!(ret_msg.instruction, CommandInst::GenHashSalt as i64);
            assert_eq!(ret_msg.argument_count, 1);
            assert_eq!(ret_msg.data_message_number, 0);
            assert_eq!(ret_msg.data_message_max, 1);
            assert_eq!(ret_msg.data.len(), digest::SHA512_OUTPUT_LEN/2);
            Ok(*array_ref!(ret_msg.data, 0, digest::SHA512_OUTPUT_LEN/2))
        },
        Err(_) => Err("AUTH_SALT_RETRIEVAL_MESSAGE_FAILED".to_string())
    }
}
