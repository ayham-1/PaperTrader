use data_encoding::HEXUPPER;
use ring::digest;
use std::io::Write;

use crate::common::account::hash::hash;
use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;
use crate::common::misc::return_flags::ReturnFlags;

use crate::client::network::cmd::req_server_salt::req_server_salt;
use crate::client::network::cmd::wait_and_read_branched::wait_and_read_branched;
use crate::client::network::tls_client::TlsClient;

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
pub fn acc_auth(
    tls_client: &mut TlsClient,
    poll: &mut mio::Poll,
    username: &str,
    email: &str,
    password: &str,
) -> Result<(), ReturnFlags> {
    /*
     * get email salt
     * */
    let email_salt: [u8; digest::SHA512_OUTPUT_LEN] =
        match req_server_salt(tls_client, poll, username, CommandInst::GetEmailSalt as i64) {
            Ok(salt) => salt,
            Err(err) => return Err(err),
        };
    /*
     * get password salt
     * */
    let password_salt: [u8; digest::SHA512_OUTPUT_LEN] = match req_server_salt(
        tls_client,
        poll,
        username,
        CommandInst::GetPasswordSalt as i64,
    ) {
        Ok(salt) => salt,
        Err(err) => return Err(err),
    };

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
    tls_client
        .write(&bincode::serialize(&message).unwrap())
        .unwrap();

    /* wait for a response */
    wait_and_read_branched(tls_client, poll, Some(15), Some(500))?;

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();

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
    ) && response.data.len() != 0
        && response.instruction == 1
    {
        /* authorized */
        tls_client.auth_jwt = match String::from_utf8(response.data) {
            Ok(token) => token,
            Err(_) => return Err(ReturnFlags::CLIENT_ACC_INVALID_SESSION_ID),
        };
        Ok(())
    } else {
        Err(ReturnFlags::CLIENT_ACC_UNAUTHORIZED)
    }
}
