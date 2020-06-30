use crate::ds::account::portfolio::Portfolio;

#[derive(PartialEq, Debug)]
pub struct Account {
    pub username: String,
    pub email: String,
    pub is_pass: bool,
    pub pass_hash: String,
    pub portfolio: Portfolio,
    pub transactions: Vec<f64>
}
