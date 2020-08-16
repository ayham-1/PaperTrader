use crate::server::db::config::{DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS};
use crate::server::db::initializer::db_connect;
use crate::common::account::position::Position;

/// Creates a position on the posttgre SQL database
///
/// Takes in the position to insert to the database.
///
/// Arguments:
/// position - The position to use.
///
/// Example:
/// ```rust
///     match create_position(Position::default()) {
///         Ok(_) => {},
///         Err(err) => panic!("TEST_CMD_CREATE_PORTFOLIO_FAILED: {}", err)
///     }
/// ```
pub fn create_position(position: Position) -> Result<(), String> {
    /*
     * Creates a position entry in database in portfolio_schema.positions.
     * */
    /* connect to database */
    let mut client = db_connect(DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS)?;

    /* insert position */
    match client.execute("INSERT INTO portfolio_schema.positions 
                         (stock_symbol, stock_open_amount, stock_open_price, stock_open_cost, stock_close_amount, 
                          stock_close_price, open_epoch, close_epoch, is_buy, is_open) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", 
                         &[&position.stock_symbol, &position.stock_open_amount, &position.stock_open_price, 
                         &position.stock_open_cost, &position.stock_close_amount, &position.stock_close_price, 
                         &position.open_epoch, &position.close_epoch, &(position.action_type as i64 != 0), 
                         &position.is_open]) {
        Ok(_rows) => Ok(()),
        Err(err) => Err(format!("CMD_CREATE_POSITION_FAILED: {}", err)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_create_position() {
        match create_position(Position::default()) {
            Ok(_) => {},
            Err(err) => panic!("TEST_CMD_CREATE_PORTFOLIO_FAILED: {}", err)
        }
    }
}
