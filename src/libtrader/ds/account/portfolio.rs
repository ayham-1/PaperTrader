use crate::ds::account::transaction::Transaction;

#[derive(PartialEq, Debug)]
pub struct Portfolio {
    pub transaction_history: Vec<Transaction>,
}
