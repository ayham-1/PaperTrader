use crate::common::account::portfolio::Portfolio;
use crate::common::account::transaction::Transaction;

#[derive(PartialEq, Debug)]
pub struct Account {
    pub username: String,

    pub email_hash: String,
    pub server_email_salt: String,
    pub client_email_salt: String,

    pub pass_hash: String,
    pub server_pass_salt: String,
    pub client_pass_salt: String,

    pub is_pass: bool,
    pub portfolio: Portfolio,
    pub transactions: Vec<Transaction>
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {:#?})", self.username, self.email_hash, self.is_pass, self.pass_hash, 
               self.portfolio, self.transactions)
    }
}
