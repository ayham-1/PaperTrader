use crate::db::config::{DB_USER, DB_PASS};
use crate::db::init::db_connect;
use crate::ds::server::global_state::GlobalState;

/*
 * Creates a stock table in database in assets schema.
 */
pub fn create_stock(state: &mut GlobalState, stock_name: String) -> Result<(), String> {
    // Connect to database.
    let mut client = db_connect(state, DB_USER, DB_PASS)?;

    // Create the table.
    match client.execute(format!("CREATE TABLE asset_schema.{} ( \
                        id                  BIGSERIAL PRIMARY KEY, \
                        isin                TEXT NOT NULL, \
                        time_since_epoch    TIMESTAMP NOT NULL, \
                        ask_price           BIGINT NOT NULL, \
                        bid_price           BIGINT NOT NULL, \
                        volume              BIGINT NOT NULL \
                )", stock_name).as_str(), &[]) {
        Ok(_rows) => Ok(()),
        Err(err) => Err(format!("DB_FAILED_TO_CREATE_STOCK_TABLE: {}", err)),
    }
}
