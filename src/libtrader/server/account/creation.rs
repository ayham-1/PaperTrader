use crate::common::message::message::Message;
use crate::common::account::portfolio::Portfolio;

use crate::server::account::hash_email::hash_email;
use crate::server::account::hash_pwd::hash_pwd;
use crate::server::ds::account::Account;
use crate::server::db::initializer::db_connect;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};

pub fn acc_create(message: &Message) -> Result<(), String> {
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
    let mut client = db_connect(DB_ACC_USER, DB_ACC_PASS)?;

    /* search for an account with same name */
    for _ in &client.query(
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
