use data_encoding::HEXUPPER;
use ring::digest;
use std::io;

use crate::common::account::hash::hash;
use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;
use crate::common::misc::return_flags::ReturnFlags;

use crate::client::network::cmd::req_server_salt::req_server_salt;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;

/// Client authentication procedure.
///
/// Takes in the username, email and password. Data is hashed and then sent to the server for
/// further hashing and confirmation of authentication. A session token is returned.
/// The function is not complete.
///
/// Currently only sends authentication request and does not process any returned values.
///
/// Arguments:
/// username - The raw username to be used.
/// email - The raw email to be used.
/// password - The raw password to be used.
///
/// Returns: nothing on success, and ReturnFlags on failure.
pub async fn acc_auth(
    socket: &mut TlsStream<TcpStream>,
    username: &str,
    email: &str,
    password: &str,
) -> io::Result<String> {
    /*
     * get email salt
     * */
    let email_salt: [u8; digest::SHA512_OUTPUT_LEN] =
        req_server_salt(socket, username, CommandInst::GetEmailSalt as i64).await?;

    /*
     * get password salt
     * */
    let password_salt: [u8; digest::SHA512_OUTPUT_LEN] =
        req_server_salt(socket, username, CommandInst::GetPasswordSalt as i64).await?;

    /*
     * hash the email
     */
    let hashed_email = hash(&email.as_bytes().to_vec(), &email_salt.to_vec(), 175_000);

    /*
     * hash the password
     */
    let hashed_password = hash(
        &password.as_bytes().to_vec(),
        &password_salt.to_vec(),
        250_000,
    );

    /* generate message to be sent to the server */
    let data = object! {
        hashed_email: HEXUPPER.encode(&hashed_email),
        hashed_password: HEXUPPER.encode(&hashed_password),
        username: username
    };
    let message = message_builder(
        MessageType::Command,
        CommandInst::LoginMethod1 as i64,
        3,
        0,
        0,
        data.dump().as_bytes().to_vec(),
    );
    socket
        .write_all(&bincode::serialize(&message).unwrap())
        .await?;

    /* decode response */
    let mut buf = Vec::with_capacity(4096);
    socket.read_buf(&mut buf).await?;
    let response: Message = bincode::deserialize(&buf).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{}", ReturnFlags::ClientAccUnauthorized),
        )
    })?;

    if assert_msg(
        &response,
        MessageType::ServerReturn,
        true,
        1,
        false,
        0,
        false,
        0,
        false,
        0,
    ) && response.data.len() != 0
        && response.instruction == 1
    {
        /* authorized */
        return Ok(String::from_utf8(response.data).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}", ReturnFlags::ClientAccInvalidSessionId),
            )
        })?);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            format!("{}", ReturnFlags::ClientAccUnauthorized),
        ));
    }
}
