use crate::db::config::{*};

use postgres::{Client, NoTls};

pub fn db_init() -> Result<Client, postgres::error::Error> {
    let client = Client::connect(format!("host={} port={} user={} db={} password={}", DB_HOST, DB_HOST_PORT, DB_USER, DB_NAME, DB_PASS).as_str(), NoTls);

    return client;
}
