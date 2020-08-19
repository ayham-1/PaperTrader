use std::io::Write;

use data_encoding::HEXUPPER;
use ring::{pbkdf2};
use std::num::NonZeroU32;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::db::cmd::get_user_salt::get_user_salt;
use crate::server::db::cmd::get_user_hash::get_user_hash;
use crate::server::db::cmd::get_user_id::get_user_id;

use crate::server::network::jwt_wrapper::create_jwt_token;

pub fn acc_auth(tls_connection: &mut TlsConnection, message: &Message) -> Result<(), String> { 
    /*
     * Parse account data.
     * */
    /* get json data */
    let stringified_data = std::str::from_utf8(&message.data).unwrap();
    let data = json::parse(&stringified_data).unwrap();
    /* get email, password, and username hashes */
    let email_hash = HEXUPPER.decode(data["hashed_email"].as_str().unwrap().as_bytes()).unwrap();
    let password_hash = HEXUPPER.decode(data["hashed_password"].as_str().unwrap().as_bytes()).unwrap();
    let username = data["username"].as_str().unwrap();

    /*
     * Get server salts
     * */
    let email_salt = HEXUPPER.decode(get_user_salt(username, true, true).unwrap().as_bytes()).unwrap();
    let password_salt = HEXUPPER.decode(get_user_salt(username, false, true).unwrap().as_bytes()).unwrap();

    /*
     * Get server hashes
     * */
    let email_db = HEXUPPER.decode(get_user_hash(username, true).unwrap().as_bytes()).unwrap();
    let password_db = HEXUPPER.decode(get_user_hash(username, false).unwrap().as_bytes()).unwrap();

    /*
     * Verify creds
     * */
    let email_ret = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(350_000).unwrap(),
        &email_salt,
        &email_hash,
        &email_db);
    match email_ret.is_ok() {
        true => {},
        false => return Err("ACC_AUTH_SERVER_UNAUTHORIZED1".to_string())
    };
    let pass_ret = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(500_000).unwrap(),
        &password_salt,
        &password_hash,
        &password_db);
    match pass_ret.is_ok() {
        true => {},
        false => return Err("ACC_AUTH_SERVER_UNAUTHORIZED".to_string())
    };

    /* 
     * Generate JWT token 
     * */
    /* get user id*/
    let user_id = get_user_id(username)?;

    /* gen the actual token */
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    let beginning_of_time = SystemTime::now() + Duration::from_secs(4*60*60);
    let jwt_token = create_jwt_token(user_id, 
                                     beginning_of_time.duration_since(UNIX_EPOCH).unwrap().as_secs())?;

    /* 
     * Send the JWT token 
     * */
    let message = message_builder(MessageType::ServerReturn, 1, 1, 0, 1, jwt_token.as_bytes().to_vec());
    let _ = tls_connection.write(bincode::serialize(&message).unwrap().as_slice());
    
    Ok(()) 
}
