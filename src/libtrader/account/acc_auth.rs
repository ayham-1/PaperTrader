use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use std::io::Write;

use crate::network::tls_client::TlsClient;

use crate::parser::message_builder::message_builder;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::inst::CommandInst;

/// INCOMPLETE: authenticate user.
///
/// Takes in the username, email and password. Data is hashed and then sent to the server for
/// further hashing and confirmation of authentication. A session token is returned.
/// The function is not complete. NOT TO BE USED ANYWHERE.
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
    let rng = rand::SystemRandom::new();

    /*
     * hash the email
     */
    /* generate false client salt TODO: get salt from the server */
    let mut email_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut email_client_salt).unwrap();

    let mut hashed_email = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(175_000).unwrap(),
        &email_client_salt,
        email.as_bytes(),
        &mut hashed_email);

    /*
     * hash the password
     */
    /* generate false client salt TODO: get salt from the server */
    let mut password_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut password_client_salt).unwrap();

    let mut hashed_password = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(250_000).unwrap(),
        &password_client_salt,
        password.as_bytes(),
        &mut hashed_password);

    /* TODO: send the data to the server and return a session id */

    Ok(())
}

pub fn acc_auth_server() -> Result<(), String> { Ok(()) }
