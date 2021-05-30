use ring::digest;

use std::io;

use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::return_flags::ReturnFlags;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;

/// Issues a command to the connected TLS server to obtain a salt.
///
/// All salts returned are of size ```digest::SHA512_OUTPUT_LEN/2``` or 32 bytes.
///
/// Arguments:
/// socket - The TLS connection to use for the salt.
///
/// Returns: a [u8; 32] on success, and ```ReturnFlags``` on error containing the reason of failure.
///
/// Example:
/// ```rust
///     let server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = match get_server_salt(tls_client, poll) {
///         Ok(salt) => salt,
///         Err(err) => panic!("could not retrieve server salt; err: {}", errj)
///     };
/// ```
pub async fn get_server_salt(
    socket: &mut TlsStream<TcpStream>,
) -> io::Result<[u8; digest::SHA512_OUTPUT_LEN / 2]> {
    /*
     * request to generate a salt from the server.
     * */
    let message = message_builder(
        MessageType::Command,
        CommandInst::GenHashSalt as i64,
        0,
        0,
        0,
        Vec::new(),
    );
    socket
        .write_all(&bincode::serialize(&message).unwrap())
        .await?;

    let mut buf = Vec::with_capacity(4096);
    socket.read_buf(&mut buf).await?;

    let ret_msg: Message = bincode::deserialize(&buf).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{}", ReturnFlags::ClientGenSaltFailed),
        )
    })?;

    // TODO: crash this thread LOVELY ERROR CHECKING!
    assert_eq!(ret_msg.msgtype, MessageType::DataTransfer);
    assert_eq!(ret_msg.instruction, CommandInst::GenHashSalt as i64);
    assert_eq!(ret_msg.argument_count, 1);
    assert_eq!(ret_msg.data_message_number, 0);
    assert_eq!(ret_msg.data_message_max, 1);
    assert_eq!(ret_msg.data.len(), digest::SHA512_OUTPUT_LEN / 2);
    Ok(*array_ref!(ret_msg.data, 0, digest::SHA512_OUTPUT_LEN / 2))
}
