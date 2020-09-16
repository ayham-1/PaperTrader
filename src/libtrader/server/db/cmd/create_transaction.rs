use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};
use crate::server::db::initializer::db_connect;
use crate::common::account::transaction::Transaction;
use crate::common::misc::return_flags::ReturnFlags;

/// Creates a trasnaction on the postgre SQL database
///
/// Takes in the transaction and a userId to insert to the database.
///
/// Arguments:
/// user_id - ID to use for the new transaction.
/// transaction - The transaction to use.
///
/// Example:
/// ```rust
///     match create_transaction(Position::default()) {
///         Ok(_) => {},
///         Err(err) => panic!("TEST_CMD_CREATE_TRANSACTION_FAILED: {}", err)
///     }
/// ```
pub fn create_transaction(user_id: i64, transaction: &Transaction) -> Result<(), ReturnFlags> {
    /*
     * Creates a transaction entry in database in accounts_schema.transactions.
     * */
    /* connect to database */
    let mut client = db_connect(DB_ACC_USER, DB_ACC_PASS)?;

    /* insert position */
    match client.execute("INSERT INTO accounts_schema.transactions 
                         (user_id, stock_symbol, shares_size, shares_cost, is_buy) 
                         VALUES ($1, $2, $3, $4, $5)",
                         &[&user_id, &transaction.stock_symbol, &transaction.shares_size, &transaction.shares_cost,
                         &transaction.is_buy]) {
        Ok(_rows) => Ok(()),
        Err(_) => Err(ReturnFlags::SERVER_DB_CREATE_TRANSACTION_FAILED),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_create_transaction() {
        match create_transaction(1, &Transaction::default()) {
            Ok(_) => {},
            Err(err) => panic!("TEST_CMD_CREATE_TRANSACTION_FAILED: {}", err)
        }
    }
}
