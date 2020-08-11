use crate::db::config::{DB_USER, DB_PASS};
use crate::db::initializer::db_connect;
use crate::ds::generic::global_state::GlobalState;

/// Creates a stock on the postgres SQL database.
/// 
/// Takes in a stock name and creates a table in the ```asset_schema``` schema
///
/// Arguments:
/// state - The global state used.
/// stock_name - The name of the stock to create.
///
/// Returns: nothing on success, a string containing the reason of failure on error.
///
/// Example:
/// ```rust
///    match create_stock(&mut state, "AAPL") {
///        Ok(()) => info!("created stock table"),
///        Err(err) => error!("failed to create stock table {}", err),
///    }
/// ```
pub fn create_stock(state: &mut GlobalState, stock_name: &str) -> Result<(), String> {
    /*
     * Creates a stock table in database in assets schema.
     */
    // Connect to database.
    let mut client = db_connect(state, DB_USER, DB_PASS)?;

    // Create the table.
    match client.execute(format!("CREATE TABLE asset_schema.{} ( \
                        id                  BIGSERIAL PRIMARY KEY, \
                        isin                TEXT NOT NULL, \
                        time_epoch          BIGINT NOT NULL, \
                        ask_price           BIGINT NOT NULL, \
                        bid_price           BIGINT NOT NULL, \
                        volume              BIGINT NOT NULL \
                )", stock_name).as_str(), &[]) {
        Ok(_rows) => Ok(()),
        Err(err) => Err(format!("DB_FAILED_TO_CREATE_STOCK_TABLE: {}", err)),
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cmd_create_stock() {
        /* create global state */
        let mut state: GlobalState = GlobalState::default();
        
        /* test create_stock() */
        match create_stock(&mut state, "AAPL") {
            Ok(()) => {
                /* connect to db */
                let mut client = db_connect(&mut state, DB_USER, DB_PASS).unwrap();

                /* confirm that stock table was created */
                match client.query("SELECT EXISTS ( \
                            SELECT FROM information_schema.tables \
                            WHERE table_schema = 'asset_schema' \
                            AND table_name = 'aapl')", &[]) {
                    Ok(rows) => {
                        let exists: bool = rows[0].get(0);
                        assert_eq!(exists, true);
                    },
                    Err(err) => panic!("TEST_CMD_CREATE_STOCK: {}", err)
                }
            },
            Err(err) => panic!("TEST_CMD_CREATE_STOCK: {}", err)
        }
    }
}
