use crate::server::db::initializer::db_connect;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};

use crate::server::db::cmd::user_exists::user_exists;

pub fn get_user_salt(username: &str, is_email: bool, is_server: bool) -> Result<String, String> {
    /* check that user exists*/
    if user_exists(username) {
        let mut client = db_connect(DB_ACC_USER, DB_ACC_PASS)?;
        if is_server {
            if is_email {
                for row in 
                    &client.query("SELECT username, server_email_salt FROM accounts_schema.accounts WHERE username LIKE $1", 
                                  &[&username]).unwrap() {
                        return Ok(row.get(1));
                    }
            } else {
                for row in 
                    &client.query("SELECT username, server_pass_salt FROM accounts_schema.accounts WHERE username LIKE $1", 
                                  &[&username]).unwrap() {
                        return Ok(row.get(1));
                    }
            }
        } else {
            if is_email {
                for row in 
                    &client.query("SELECT username, client_email_salt FROM accounts_schema.accounts WHERE username LIKE $1", 
                                  &[&username]).unwrap() {
                        return Ok(row.get(1));
                    }
            } else {
                for row in 
                    &client.query("SELECT username, client_pass_salt FROM accounts_schema.accounts WHERE username LIKE $1", 
                                  &[&username]).unwrap() {
                        return Ok(row.get(1));
                    }
            }
        }
    }

    Err("GET_USER_SALT_NOT_FOUND".to_string())
}
