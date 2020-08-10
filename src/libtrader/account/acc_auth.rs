use std::io::Write;

use crate::account::hash::hash;

use crate::network::tls_client::TlsClient;
use crate::network::cmd::client::req_server_salt::req_server_salt;
use crate::network::cmd::generic::wait_and_read_branched::wait_and_read_branched;

use crate::parser::message_builder::message_builder;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::message::Message;
use crate::ds::message::inst::CommandInst;

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
/// Returns: nothing.
pub fn acc_auth_client(tls_client: &mut TlsClient, poll: &mut mio::Poll,
                       username: &str, email: &str, password: &str) -> Result<(), String> {
    /*
     * get email salt
     * */
    let email_salt = match req_server_salt(tls_client, poll, username, CommandInst::GetEmailSalt as i64) {
        Ok(salt) => salt,
        Err(err) => return Err(format!("ACC_AUTH_CLIENT_COULD_NOT_GET_SALT: {}", err))
    };
    /*
     * get password salt
     * */
    let password_salt = match req_server_salt(tls_client, poll, username, CommandInst::GetPasswordSalt as i64) {
        Ok(salt) => salt,
        Err(err) => return Err(format!("ACC_AUTH_CLIENT_COULD_NOT_GET_SALT: {}", err))
    };

    println!("khello we are here");

    /*
     * hash the email
     */
    let hashed_email = hash(email, email_salt.to_vec(), 175_000);

    /*
     * hash the password
     */
    let hashed_password = hash(password, password_salt.to_vec(), 250_000);

    /* generate message to be sent to the server */
    let mut data = Vec::new();
    data.append(&mut bincode::serialize(&email_salt.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&hashed_email.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&password_salt.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&hashed_password.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&username.as_bytes()).unwrap());
    match message_builder(MessageType::Command, CommandInst::LoginMethod1 as i64, 5, 0, 0, data) {
        Ok(message) => {
            tls_client.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();

            /* wait for a response */
            wait_and_read_branched(tls_client, poll, Some(15), Some(500))?;
        },
        Err(_) => return Err("ACC_AUTH_CLIENT_COULD_NOT_BUILD_MESSAGE".to_string())
    };

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();

    if response.msgtype == MessageType::ServerReturn && response.instruction == 1 
        && response.argument_count == 1 && response.data.len() != 0 {
            /* authorized */
            tls_client.auth_jwt = match String::from_utf8(response.data) {
                Ok(token) => token,
                Err(err) => return Err(format!("ACC_AUTH_CLIENT_INVALID_SESSION_ID: {}", err)),
            };
            Ok(())
        } else {
            Err("ACC_AUTH_CLIENT_UNAUTHORIZED".to_string())
        }

}

pub fn acc_auth_server() -> Result<(), String> { Ok(()) }
