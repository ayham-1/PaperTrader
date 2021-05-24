use data_encoding::HEXUPPER;
use ring::pbkdf2;
use std::num::NonZeroU32;

use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::db::cmd::get_user_hash::get_user_hash;
use crate::server::db::cmd::get_user_id::get_user_id;
use crate::server::db::cmd::get_user_salt::get_user_salt;

use crate::server::network::jwt_wrapper::create_jwt_token;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn acc_auth(
    tls_connection: &mut TlsStream<TcpStream>,
    message: &Message,
) -> Result<(), ReturnFlags> {
    /*
     * Parse account data.
     * */
    /* get json data */
    let stringified_data = std::str::from_utf8(&message.data).unwrap();
    let data = json::parse(&stringified_data).unwrap();
    /* get email, password, and username hashes */
    let email_hash = HEXUPPER
        .decode(data["hashed_email"].as_str().unwrap().as_bytes())
        .unwrap();
    let password_hash = HEXUPPER
        .decode(data["hashed_password"].as_str().unwrap().as_bytes())
        .unwrap();
    let username = data["username"].as_str().unwrap();

    /*
     * Get server salts
     * */
    let email_salt = HEXUPPER
        .decode(get_user_salt(username, true, true).unwrap().as_bytes())
        .unwrap();
    let password_salt = HEXUPPER
        .decode(get_user_salt(username, false, true).unwrap().as_bytes())
        .unwrap();

    /*
     * Get server hashes
     * */
    let email_db = HEXUPPER
        .decode(get_user_hash(username, true).unwrap().as_bytes())
        .unwrap();
    let password_db = HEXUPPER
        .decode(get_user_hash(username, false).unwrap().as_bytes())
        .unwrap();

    /*
     * Verify creds
     * */
    let email_ret = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(350_000).unwrap(),
        &email_salt,
        &email_hash,
        &email_db,
    );
    match email_ret.is_ok() {
        true => {}
        false => return Err(ReturnFlags::ServerAccUnauthorized),
    };
    let pass_ret = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(500_000).unwrap(),
        &password_salt,
        &password_hash,
        &password_db,
    );
    match pass_ret.is_ok() {
        true => {}
        false => return Err(ReturnFlags::ServerAccUnauthorized),
    };

    /*
     * Generate JWT token
     * */
    /* get user id*/
    let user_id = get_user_id(username)?;

    /* gen the actual token */
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    let beginning_of_time = SystemTime::now() + Duration::from_secs(4 * 60 * 60);
    let jwt_token = create_jwt_token(
        user_id,
        beginning_of_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )?;

    /*
     * Send the JWT token
     * */
    let message = message_builder(
        MessageType::ServerReturn,
        1,
        1,
        0,
        0,
        jwt_token.as_bytes().to_vec(),
    );
    tls_connection
        .write_all(bincode::serialize(&message).unwrap().as_slice())
        .await
        .expect("could not write to client");

    Ok(())
}
