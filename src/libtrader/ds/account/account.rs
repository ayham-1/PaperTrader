use crate::ds::account::portfolio::Portfolio;

#[derive(PartialEq, Debug)]
pub struct Account {
    pub username: String,
    pub server_username_salt: String,
    pub client_username_salt: String,

    pub email: String,
    pub server_email_salt: String,
    pub client_email_salt: String,

    pub pass_hash: String,
    pub server_pass_salt: String,
    pub client_pass_salt: String,

    pub is_pass: bool,
    pub portfolio: Portfolio,
    pub transactions: Vec<f64>
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {:#?})", self.username, self.email, self.is_pass, self.pass_hash, 
               self.portfolio, self.transactions)
    }
}
