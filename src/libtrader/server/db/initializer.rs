use crate::server::db::config::{DB_HOST, DB_HOST_PORT, DB_NAME};
use crate::common::misc::return_flags::ReturnFlags;

/// Establishes a postgresql connection to the SQL database.
///
/// Creates a postgresql connection. 
///
/// Arguments:
/// user - The name of the user to connect to the database with.
/// pass - The password of the user to connect to the database with.
///
/// Returns: ```postgres::Client``` on success, and a string containing the
/// reason of failure on error.
///
/// Example:
/// ```rust
/// let mut client = db_connect(DB_USER, DB_PASS)?;
/// ```
pub fn db_connect(user: &'static str, pass: &'static str) -> Result<postgres::Client, ReturnFlags> {
    /* Generate the requested string */
    let db_connect_str = format!("host={} port={} dbname={} user={} password={}",
                                 DB_HOST, DB_HOST_PORT, DB_NAME, user, pass);
    match postgres::Client::connect(db_connect_str.as_str(), postgres::NoTls) {
        Ok(client) => return Ok(client),
        Err(_) => return Err(ReturnFlags::SERVER_DB_CONNECT_FAILED)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::server::db::config::{DB_USER, DB_PASS};
    #[test]
    fn test_db_connect() {
        match db_connect(DB_USER, DB_PASS) {
            Ok(client) => assert_eq!(client.is_closed(), false),
            Err(err) => panic!("TEST_DB_CONNECT_FAILED: {}", err),
        }
    }
}
