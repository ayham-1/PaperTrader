use data_encoding::HEXUPPER;
use ring::digest;
use std::io::Write;

use crate::client::account::hash_email::hash_email;
use crate::client::account::hash_pwd::hash_pwd;

use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::return_flags::ReturnFlags;

use crate::client::network::cmd::get_server_salt::get_server_salt;
use crate::client::network::cmd::wait_and_read_branched::wait_and_read_branched;
use crate::client::network::tls_client::TlsClient;


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
pub fn acc_create(
    tls_client: &mut TlsClient,
    poll: &mut mio::Poll,
    username: &str,
    email: &str,
    password: &str,
) -> Result<(), ReturnFlags> {
    /*
     * get three server salts for email, and password
     * */
    let email_server_salt: [u8; digest::SHA512_OUTPUT_LEN / 2] =
        match get_server_salt(tls_client, poll) {
            Ok(salt) => salt,
            Err(_) => return Err(ReturnFlags::CLIENT_REQ_SALT_FAILED),
        };
    let password_server_salt: [u8; digest::SHA512_OUTPUT_LEN / 2] =
        match get_server_salt(tls_client, poll) {
            Ok(salt) => salt,
            Err(_) => return Err(ReturnFlags::CLIENT_REQ_SALT_FAILED),
        };

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
    tls_client
        .write(&bincode::serialize(&message).unwrap())
        .unwrap();

    /* wait for response */
    wait_and_read_branched(tls_client, poll, None, None)?;

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();

    // TODO: fix this garbage of message checking
    if response.msgtype == MessageType::ServerReturn && response.instruction == 1 {
        /* created successfully */
        return Ok(());
    } else {
        /* server rejected account creation */
        return Err(ReturnFlags::CLIENT_ACC_CREATION_FAILED); // TODO: return server returned error too
    }
}
