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

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {:#?})", self.username, self.email, self.is_pass, self.pass_hash, self.portfolio, self.transactions)
    }
}
