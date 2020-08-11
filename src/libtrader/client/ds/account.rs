use crate::ds::account::portfolio::Portfolio;
use crate::ds::account::transaction::Transaction;

#[derive(PartialEq, Debug)]
pub struct Account {
    pub username: String,

    pub portfolio: Portfolio,
    pub transactions: Vec<Transaction>
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {:#?})", self.username, self.email_hash, self.is_pass, self.pass_hash, 
               self.portfolio, self.transactions)
    }
}
