use libtrader::ds::account::account::Account;
use libtrader::ds::account::portfolio::Portfolio;

use postgres::{Client, NoTls};

fn main() {
    let account = Account{
        username: "test".to_string(),
        email: "email".to_string(), 
        is_pass: true,
        pass_hash: "passhash".to_string(),
        portfolio: Portfolio{
            position_history: Vec::new(),
            open_positions: Vec::new()
        },
        transactions: Vec::new()
    };

    let mut client = Client::connect("host=localhost port=5432 user=pt_usr password=test dbname=pt_db", NoTls).unwrap();
    client.batch_execute("
    CREATE TABLE accounts (
        id      SERIAL PRIMARY KEY,
        acct    CHAR
    )").unwrap();

    println!("Hello World!");
}

