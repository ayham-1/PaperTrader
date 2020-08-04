use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

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
pub fn acc_auth_client(username: &str, email: &str, password: &str) -> Result<(), ()> {
    let rng = rand::SystemRandom::new();

    /* 
     * hash the username 
     * */
    /* generate false client salt TODO: get salt from the server */
    let mut user_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut user_client_salt).unwrap();

    let mut hashed_usr = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &user_client_salt,
        username.as_bytes(),
        &mut hashed_usr);

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
