use postgres_types::{ToSql, FromSql};

use crate::ds::account::portfolio::Portfolio;

#[derive(PartialEq, Debug, ToSql, FromSql)]
pub struct Account {
    pub username: String,
    pub email: String,
    pub is_pass: bool,
    pub pass_hash: String,
    pub portfolio: Portfolio,
    pub transactions: Vec<f64>
}
