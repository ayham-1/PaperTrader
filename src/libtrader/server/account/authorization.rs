use std::io::Write;

use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;
use crate::common::account::portfolio::Portfolio;

use crate::server::account::hash::hash;
use crate::server::network::tls_connection::TlsConnection;
use crate::server::ds::account::Account;
use crate::server::db::initializer::db_connect;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};
use crate::server::db::cmd::get_user_salt::get_user_salt;
use crate::server::db::cmd::get_user_hash::get_user_hash;

use crate::server::network::jwt_wrapper::create_jwt_token;

pub fn acc_auth(tls_connection: &mut TlsConnection, message: &Message) -> Result<(), String> { 
    /*
     * Parse account data.
     * */
    /* get json data */
    let stringified_data = std::str::from_utf8(&message.data).unwrap();
    let data = json::parse(&stringified_data).unwrap();
    /* get email, password, and username hashes */
    let email_hash = data["hashed_email"].as_str().unwrap();
    let password_hash = data["hashed_password"].as_str().unwrap();
    let username = data["username"].as_str().unwrap();

    /*
     * Get server salts
     * */
    let email_salt = get_user_salt(username, true, true).unwrap().as_bytes().to_vec();
    let password_salt = get_user_salt(username, false, true).unwrap().as_bytes().to_vec();

    /*
     * Get hashes
     * */
    let email_db = get_user_hash(username, true).unwrap().as_bytes().to_vec();
    let password_db = get_user_hash(username, false).unwrap().as_bytes().to_vec();

    /*
     * Verify creds
     * */
    let email_ret = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(350_000).unwrap(),
        &email_salt,
        &email_db,
        &email_hash.as_bytes());
    match email_ret.is_ok() {
        true => {},
        false => return Err("ACC_AUTH_SERVER_UNAUTHORIZED".to_string())
    };
    let pass_ret = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(500_000).unwrap(),
        &password_salt,
        &password_db,
        &password_hash.as_bytes());
    match pass_ret.is_ok() {
        true => {},
        false => return Err("ACC_AUTH_SERVER_UNAUTHORIZED".to_string())
    };

    /* 
     * Generate JWT token 
     * */
    /* get user id*/
    let mut data = db_connect(DB_ACC_USER, DB_ACC_PASS)?;
    let mut user_id: i64 = 0;
    for row in data.query("SELECT id,username FROM accounts_schema WHERE username LIKe $1", &[&username]).unwrap() {
        user_id = row.get(0);
    }
    assert_eq!(user_id > 0, true);

    /* gen the actual token */
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    let beginning_of_time = SystemTime::now() + Duration::from_secs(4*60*60);
    let jwt_token = create_jwt_token(user_id as usize, 
                                     beginning_of_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize)?;

    /* 
     * Send the JWT token 
     * */
    match message_builder(MessageType::ServerReturn, 1, 1, 0, 1, jwt_token.as_bytes().to_vec()) {
        Ok(message) => {
            match tls_connection.tls_session.write(bincode::serialize(&message).unwrap().as_slice()) {
                Ok(_) => return Ok(()),
                Err(err) => {warn!("ACC_AUTH_FAILED_SENDING_RESPONSE: {}", err); tls_connection.closing = true;}
            }
        },
        Err(_) => return Err("ACC_AUTH_SERVER_COULD_NOT_BUILD_MESSAGE".to_string())
    };
    
    Ok(()) 
}
