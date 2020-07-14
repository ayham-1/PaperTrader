use crate::db::config::{*};

use postgres::{Client, NoTls};

pub fn init() -> Result< Client, Error> {
    let mut client = Client::connect(format!("host={} port={} user={} db={} password={}", DB_HOST, DB_HOST_PORT, DB_USER, DB_PASS), NoTls);

    return client;
}
