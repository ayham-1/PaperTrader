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

/// Issues a command to the connected TLS server to obtain a stored salt for either email or
/// password.
///
/// All salts returned are of size ```digest::SHA512_OUTPUT_LEN``` or 64 bytes.
/// Should be used in contexts that return ```io::Result```.
/// Should be used in Async contexts.
///
/// Arguments:
/// socket - The TLS client to use for the salt.
/// username - The username to obtain the salt.
/// salt_type - The CommmandInst, either GetEmailSalt, or GetPasswordSalt.
///
/// Returns: a ```io::Result<[u8; 64]>```.
/// Example:
/// ```rust
///     let server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = req_server_salt(tls_client, "n1ckn8me",
///                                                                          CommandInst::GetEmailSalt)?;
/// ```
pub async fn req_server_salt(
    socket: &mut TlsStream<TcpStream>,
    username: &str,
    salt_type: i64,
) -> io::Result<[u8; digest::SHA512_OUTPUT_LEN]> {
    /* enforce salt_type to be either email or password */
    assert_eq!(salt_type >= CommandInst::GetEmailSalt as i64, true);
    assert_eq!(salt_type <= CommandInst::GetPasswordSalt as i64, true);

    /* generate message to send */
    let message = message_builder(
        MessageType::Command,
        salt_type,
        1,
        0,
        0,
        username.as_bytes().to_vec(),
    );
    socket
        .write_all(&bincode::serialize(&message).unwrap())
        .await?;

    let mut buf = Vec::with_capacity(4096);
    socket.read_buf(&mut buf).await?;

    let ret_msg: Message = bincode::deserialize(&buf).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{}", ReturnFlags::ClientReqSaltInvMsg),
        )
    })?;

    match ret_msg.msgtype {
        MessageType::Command => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}", ReturnFlags::ClientReqSaltInvMsg),
        )),
        MessageType::DataTransfer => {
            if ret_msg.data.len() != digest::SHA512_OUTPUT_LEN {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}", ReturnFlags::ClientReqSaltInvMsgRetSize),
                ))
            } else if ret_msg.instruction == salt_type {
                Ok(*array_ref!(ret_msg.data, 0, digest::SHA512_OUTPUT_LEN))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}", ReturnFlags::ClientReqSaltInvMsgInst),
                ))
            }
        }
        MessageType::ServerReturn => match ret_msg.instruction {
            0 => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}", ReturnFlags::ClientReqSaltRej),
            )),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}", ReturnFlags::ClientReqSaltInvMsg),
            )),
        },
    }
}
