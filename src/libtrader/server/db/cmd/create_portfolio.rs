use crate::server::db::config::{DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS};
use crate::server::db::initializer::db_connect;

/// Creates a portfolio on the postgres SQL database
///
/// Takes in a user id and writes an entry portfolio for that user.
///
/// Arguments:
/// user_id - The user id to create the portfolio for.
///
/// Example:
/// ```rust
///     match create_portfolio(1) {
///         Ok(portfolio) => {},
///         Err(err) => panic!("TEST_CMD_CREATE_PORTFOLIO_FAILED: {}", err)
///     }
/// ```
pub fn create_portfolio(user_id: i64) -> Result<(), String> {
    /*
     * Creates a portfolio entry in database in portfolio_schema.portfolios.
     * */
    /* Connect to database */
    let mut client = db_connect(DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS)?;

    /* insert portfolio */
    match client.execute("INSERT INTO portfolio_schema.portfolios (userid) VALUES ($1)", &[&user_id]) {
        Ok(_row) => Ok(()),
        Err(err) => Err(format!("CMD_CREATE_PORTFOLIO_FAILED: {}", err))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_create_portfolio() {
        match create_portfolio(1) {
            Ok(_) => {},
            Err(err) => panic!("TEST_CMD_CREATE_PORTFOLIO_FAILED: {}", err)
        }
    }
}
