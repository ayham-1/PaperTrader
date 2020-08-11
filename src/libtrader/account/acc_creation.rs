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
/// Returns: nothing on success, a string on error containing the reason of failure.
///
/// Example:
/// ```rust
///     match acc_create(&mut tlsclient, &mut  poll, "test", "test", "test") {
///         Ok(()) => println!("server returned yes"),
///         Err(err) => panic!("panik {}", err),
///     }
/// ```
#[allow(unused_imports)]
use crate::network::tls_client::TlsClient;
#[cfg(feature="client")]
pub fn acc_create(tls_client: &mut TlsClient, poll: &mut mio::Poll, 
                  username: &str, email: &str, password: &str) -> Result<(), String> {
    use ring::rand::SecureRandom;
    use ring::{digest, rand};
    use std::io::Write;

    use crate::account::hash::hash;

    use crate::network::cmd::client::get_server_salt::get_server_salt;

    use crate::parser::message_builder::message_builder;
    use crate::ds::message::message::Message;
    use crate::ds::message::message_type::MessageType;
    use crate::ds::message::inst::{CommandInst};

    use crate::network::cmd::generic::wait_and_read_branched::wait_and_read_branched;

    /*
     * get three server salts for email, and password
     * */
    let email_server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = match get_server_salt(tls_client, poll) {
        Ok(salt) => salt,
        Err(err) => return Err(format!("ACC_CREATE_RETRIEVE_SALTS_FAILED: {}", err))
    };
    let password_server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = match get_server_salt(tls_client, poll) {
        Ok(salt) => salt,
        Err(err) => return Err(format!("ACC_CREATE_RETRIEVE_SALTS_FAILED: {}", err))
    };

    /*
     * generate client salts for email, password
     * */
    let rng = rand::SystemRandom::new();
    let mut email_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut email_client_salt).unwrap();
    let mut password_client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut password_client_salt).unwrap();

    /*
     * generate final salts for email, password
     * */
    let email_salt = [email_server_salt, email_client_salt].concat();
    let password_salt = [password_server_salt, password_client_salt].concat();

    /*
     * generate hashes for email, password
     * */
    let email_hash = hash(email, email_salt, 175_000);
    let password_hash = hash(password, password_salt, 250_000);

    /* generate message to be sent to the server */
    let mut data = Vec::new();
    data.append(&mut bincode::serialize(&email_hash.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&email_client_salt.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&password_hash.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&password_client_salt.to_vec()).unwrap());
    data.append(&mut bincode::serialize(&username.as_bytes()).unwrap());
    match message_builder(MessageType::Command, CommandInst::Register as i64, 6, 0, 0, data) {
        Ok(message) => {
            tls_client.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();
        },
        Err(_) => return Err("ACC_CREATE_MESSAGE_BUILD_FAILED".to_string())
    };

    /* wait for response */
    wait_and_read_branched(tls_client, poll, Some(15), Some(500))?;

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();
    
    if response.msgtype == MessageType::ServerReturn && response.instruction == 1 {
        /* created successfully */
        return Ok(());
    } else {
        /* server rejected account creation */
        return Err("ACC_CREATE_FAILED_SERVER_REJECTED".to_string());
    }
}

#[allow(unused_imports)]
use crate::network::tls_connection::TlsConnection;
use crate::ds::message::message::Message;
use crate::ds::generic::global_state::GLOBAL_STATE;
#[cfg(feature="server")]
pub fn acc_create(tls_connection: &mut TlsConnection, 
                  message: &Message) -> Result<(), String> {
    use crate::ds::account::portfolio::Portfolio;
    use crate::ds::account::account::Account;
    use crate::db::initializer::db_connect;
    use crate::db::config::{DB_ACC_USER, DB_ACC_PASS};

    use crate::account::hash_email::hash_email;
    use crate::account::hash_pwd::hash_pwd;

    /*
     * Parse account data
     * */
    /* get email, password salts and client hashes */
    let email_hash: Vec<u8> = bincode::deserialize(&message.data[..64]).unwrap();
    let email_client_salt: Vec<u8> = bincode::deserialize(&message.data[64..96]).unwrap();
    let password_hash: Vec<u8> = bincode::deserialize(&message.data[96..160]).unwrap();
    let password_client_salt: Vec<u8> = bincode::deserialize(&message.data[160..192]).unwrap();

    /* get username */
    let username: Vec<u8> = bincode::deserialize(&message.data[192..]).unwrap();

    /* generate account struct */
    let mut account: Account = Account {
        username: String::from_utf8(username).unwrap(),

        email_hash: "".to_string(),
        server_email_salt: "".to_string(),
        client_email_salt: String::from_utf8(email_client_salt).unwrap(),

        pass_hash: "".to_string(),
        server_pass_salt: "".to_string(),
        client_pass_salt: String::from_utf8(password_client_salt).unwrap(),

        is_pass: true,
        portfolio: Portfolio::default(),
        transactions: Vec::new(),
    };

    /* 
     * check if username is available in the database 
     * */
    /* connect to database */
    let mut client = db_connect(&mut GLOBAL_STATE.lock().unwrap(), DB_ACC_USER, DB_ACC_PASS)?;

    /* search for an account with same name */
    for row in &client.query(
        "SELECT username FROM accounts_schema.accounts WHERE username IS ($1)", &[&account.username]).unwrap() {
        return Err("ACC_CREATE_FAILED_USERNAME_EXISTS".to_string());
    }

    /*
     * Hash the email and password.
     * */
    /* hash the email */
    let email_server_hash = hash_email(String::from_utf8(email_hash).unwrap().as_str()).unwrap();
    account.email_hash = String::from_utf8(email_server_hash.0.to_vec()).unwrap();
    account.server_email_salt = String::from_utf8(email_server_hash.1.to_vec()).unwrap();
    /* hash the password */
    let password_server_hash = hash_pwd(String::from_utf8(password_hash).unwrap().as_str()).unwrap();
    account.pass_hash = String::from_utf8(password_server_hash.0.to_vec()).unwrap();
    account.server_pass_salt = String::from_utf8(password_server_hash.1.to_vec()).unwrap();

    /*
     * Write the account to the database.
     * */
    match client.execute("INSERT INTO accounts_schema.accounts \
        (username, email, server_email_salt, client_email_salt, pass, server_pass_salt, client_pass_salt)
        VALUES \
        ($1, $2, $3, $4, $5, $6, $7)", &[]) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(format!("ACC_CREATE_FAILED_SAVING: {}", err)),
    }
}
