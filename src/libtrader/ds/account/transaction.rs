#[derive(PartialEq, Debug)]
pub enum TransactionType { Sell, Buy }
impl Default for TransactionType {
    fn default() -> Self { TransactionType::Buy }
}

#[derive(Default, PartialEq, Debug)]
pub struct Transaction {
    pub action_type: TransactionType,
    pub cost: i64,
    pub stocks_amount: i64,
}
