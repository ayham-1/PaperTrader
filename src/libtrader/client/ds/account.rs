use crate::common::account::portfolio::Portfolio;
use crate::common::account::transaction::Transaction;

#[derive(PartialEq, Debug)]
pub struct Account {
    pub username: String,

    pub portfolio: Portfolio,
    pub transactions: Vec<Transaction>,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {:#?})",
            self.username, self.portfolio, self.transactions
        )
    }
}
