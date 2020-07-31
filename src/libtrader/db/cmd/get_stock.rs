use crate::db::config::{DB_USER, DB_PASS};
use crate::db::initializer::db_connect;
use crate::ds::server::global_state::GlobalState;
use crate::ds::generic::stock_val::StockVal;

/*
 * Returns all stock values from database.
 */
pub fn get_stock_from_db(state: &mut GlobalState, searched_symbol: String) -> Result<Vec<StockVal>, String> {
    // Connect to database.
    let mut client = db_connect(state, DB_USER, DB_PASS)?;

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match client.query(format!("SELECT * FROM asset_schema.{}", searched_symbol).as_str(), &[]) {
        Ok(all_rows) => {
            for row in all_rows {
                let mut val: StockVal = StockVal::default();
                val.id = row.get(0);
                val.isin = row.get(1);
                val.time_since_epoch = row.get(2);
                val.ask_price = row.get(3);
                val.bid_price = row.get(4);
                val.volume = row.get(5);
                stocks.push(val);
            }
            Ok(stocks)
        },
        Err(err) => Err(format!("DB_SEARCH_STOCK_NOT_FOUND: {}", err))
    }
}

/*
 * Returns all stock values from database since a time epoch.
 */
pub fn get_stock_from_db_since_epoch(state: &mut GlobalState, searched_symbol: String, 
                                     time_epoch: i64) -> Result<Vec<StockVal>, String> {
    // Connect to database.
    let mut client = db_connect(state, DB_USER, DB_PASS)?;

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match client.query(format!("SELECT * FROM asset_schema.{} WHERE time_since_epoch >= {}", searched_symbol, 
                               time_epoch).as_str(), &[]) {
        Ok(all_rows) => {
            for row in all_rows {
                let mut val: StockVal = StockVal::default();
                val.id = row.get(0);
                val.isin = row.get(1);
                val.time_since_epoch = row.get(2);
                val.ask_price = row.get(3);
                val.bid_price = row.get(4);
                val.volume = row.get(5);
                stocks.push(val);
            }
            Ok(stocks)
        },
        Err(err) => Err(format!("DB_SEARCH_STOCK_NOT_FOUND: {}", err))
    }
}

/*
 * Returns all stock values from database between two time epochs.
 */
pub fn get_stock_from_db_between_epochs(state: &mut GlobalState, searched_symbol: String, first_time_epoch: i64, 
                                        second_time_epoch: i64) -> Result<Vec<StockVal>, String> {
    // Connect to database.
    let mut client = db_connect(state, DB_USER, DB_PASS)?;

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match client.query(format!("SELECT * FROM asset_schema.{} WHERE time_since_epoch >= {} AND time_since_epoch <= {}",
                               searched_symbol, first_time_epoch, second_time_epoch).as_str(), &[]) {
        Ok(all_rows) => {
            for row in all_rows {
                let mut val: StockVal = StockVal::default();
                val.id = row.get(0);
                val.isin = row.get(1);
                val.time_since_epoch = row.get(2);
                val.ask_price = row.get(3);
                val.bid_price = row.get(4);
                val.volume = row.get(5);
                stocks.push(val);
            }
            Ok(stocks)
        },
        Err(err) => Err(format!("DB_SEARCH_STOCK_NOT_FOUND: {}", err))
    }
}
