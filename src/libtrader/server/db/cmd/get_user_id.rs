use crate::server::db::initializer::db_connect;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};

use crate::server::db::cmd::user_exists::user_exists;

pub fn get_user_id(username: &str) -> Result<i64, String> {
    /* check that user exists */
    if user_exists(username) {
        let mut client = db_connect(DB_ACC_USER, DB_ACC_PASS)?;
        for row in client.query("SELECT id,username FROM accounts_schema.accounts WHERE username LIKE $1", &[&username]).unwrap() {
            return Ok(row.get(0));
        }
    }
    return Err("GET_USER_ID_FAILED".to_string());
}
