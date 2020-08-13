use crate::server::db::initializer::db_connect;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};

pub fn user_exists(username: &str) -> bool {
    let mut client = match db_connect(DB_ACC_USER, DB_ACC_PASS) {
        Ok(conn) => conn,
        Err(_) => return false,
    };
    
    for _ in &client.query("SELECT username FROM accounts_schema.accounts WHERE username LIKE $1", &[&username]) {
        return true;
    }
    false
}

