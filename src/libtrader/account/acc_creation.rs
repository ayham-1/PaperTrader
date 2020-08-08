use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use std::io::Write;

use crate::network::cmd::client::get_server_salt::get_server_salt;
use crate::network::tls_client::TlsClient;

use crate::parser::message_builder::message_builder;
use crate::ds::message::message::Message;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::inst::{CommandInst};

use crate::network::cmd::generic::wait_and_read_branched::wait_and_read_branched;

pub fn acc_create_client(tls_client: &mut TlsClient, poll: &mut mio::Poll, 
                  username: &str, email: &str, password: &str) -> Result<(), String> {
    /*
     * get three server salts for username, email, and password
     * */
    let username_server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = match get_server_salt(tls_client, poll) {
        Ok(salt) => salt,
        Err(err) => return Err(format!("ACC_CREATE_RETRIEVE_SALTS_FAILED: {}", err))
    };
    let email_server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = match get_server_salt(tls_client, poll) {
        Ok(salt) => salt,
        Err(err) => return Err(format!("ACC_CREATE_RETRIEVE_SALTS_FAILED: {}", err))
    };
    let password_server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = match get_server_salt(tls_client, poll) {
        Ok(salt) => salt,
        Err(err) => return Err(format!("ACC_CREATE_RETRIEVE_SALTS_FAILED: {}", err))
    };

    /*
     * generate client salts for username, email, password
     * */
    let rng = rand::SystemRandom::new();
    let mut username_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut username_client_salt).unwrap();
    let mut email_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut email_client_salt).unwrap();
    let mut password_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut password_client_salt).unwrap();

    /*
     * generate final salts for username, email, password
     * */
    let username_salt = [username_server_salt, username_client_salt].concat();
    let email_salt = [email_server_salt, email_client_salt].concat();
    let password_salt = [password_server_salt, password_client_salt].concat();

    /*
     * generate three hashes for username, email, password
     * */
    let mut username_hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &username_salt,
        username.as_bytes(),
        &mut username_hash);
    let mut email_hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(175_000).unwrap(),
        &email_salt,
        email.as_bytes(),
        &mut email_hash);
    let mut password_hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(250_000).unwrap(),
        &password_salt,
        password.as_bytes(),
        &mut password_hash);
    
    println!("generated");

    /* generate message to be sent to the server */
    let mut data = Vec::new();
    data.append(&mut bincode::serialize(&username_hash.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&username_client_salt.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&email_hash.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&email_client_salt.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&password_hash.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&password_client_salt.to_vec()).unwrap());
    match message_builder(MessageType::Command, CommandInst::Register as i64, 6, 0, 0, data) {
        Ok(message) => {
            tls_client.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();
        },
        Err(err) => return Err(format!("ACC_CREATE_MESSAGE_BUILD_FAILED: {}", err))
    };

    /* wait for response */
    wait_and_read_branched(tls_client, poll, Some(15), Some(500))?;

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();
    
    if response.message_type == MessageType::ServerReturn && response.instruction == 1 {
        /* created successfully */
        return Ok(());
    } else {
        /* server rejected account creation */
        return Err("ACC_CREATE_FAILED_SERVER_REJECTED".to_string());
    }
}
