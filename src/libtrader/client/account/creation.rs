use data_encoding::HEXUPPER;
use ring::digest;
use std::io;

use crate::client::account::hash_email::hash_email;
use crate::client::account::hash_pwd::hash_pwd;

use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;
use crate::common::misc::return_flags::ReturnFlags;

use crate::client::network::cmd::get_server_salt::get_server_salt;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;

/// Requests a TLS server to create an account.
///
/// Gets three server salts, generates three new salts, cancatenates both salts, and use the
/// concatenated salt to hash the username, email, and password. Generates a message containing the
/// hashes and sends it to the server. Waits for a response and returns.
///
/// Arguments:
/// tls_client - The TLS client to use.
/// poll - The mio::Poll to get the events from.
/// username - The username to send to the server.
/// email - The email to send to the server.
/// password - The password to send to the server.
///
/// Returns: nothing on success, ReturnFlag on error containing the reason of failure.
///
/// Example:
/// ```rust
///     match acc_create(&mut tlsclient, &mut  poll, "test", "test", "test") {
///         Ok(()) => println!("server returned yes"),
///         Err(err) => panic!("panik {}", err),
///     }
/// ```
pub async fn acc_create(
    socket: &mut TlsStream<TcpStream>,
    username: &str,
    email: &str,
    password: &str,
) -> io::Result<()> {
    /*
     * get two server salts for email, and password
     * */
    let email_server_salt: [u8; digest::SHA512_OUTPUT_LEN / 2] = get_server_salt(socket).await?;
    let password_server_salt: [u8; digest::SHA512_OUTPUT_LEN / 2] = get_server_salt(socket).await?;

    /*
     * generate hashes for email, password
     * */
    let email_hash = hash_email(&email.as_bytes().to_vec(), email_server_salt);
    let password_hash = hash_pwd(&password.as_bytes().to_vec(), password_server_salt);

    /* generate message to be sent to the server */
    let data = object! {
        email_hash: HEXUPPER.encode(&email_hash.0),
        email_client_salt: HEXUPPER.encode(&email_hash.1),
        password_hash: HEXUPPER.encode(&password_hash.0),
        password_client_salt: HEXUPPER.encode(&password_hash.1),
        username: username
    };
    let message = message_builder(
        MessageType::Command,
        CommandInst::Register as i64,
        5,
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
            format!("{}", ReturnFlags::ClientTlsReadError),
        )
    })?;
    if !assert_msg(
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
    ) && response.instruction == 1
    {
        /* created successfully */
        return Ok(());
    } else {
        /* server rejected account creation */
        return Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            format!("{}", ReturnFlags::ClientAccCreationFailed),
        ));
    }
}
