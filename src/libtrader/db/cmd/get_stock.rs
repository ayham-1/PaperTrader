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
                val.time_epoch = row.get(2);
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
    match client.query(format!("SELECT * FROM asset_schema.{} WHERE time_epoch >= {}", searched_symbol, 
                               time_epoch).as_str(), &[]) {
        Ok(all_rows) => {
            for row in all_rows {
                let mut val: StockVal = StockVal::default();
                val.id = row.get(0);
                val.isin = row.get(1);
                val.time_epoch = row.get(2);
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
    match client.query(format!("SELECT * FROM asset_schema.{} WHERE time_epoch >= {} AND time_epoch <= {}",
                               searched_symbol, first_time_epoch, second_time_epoch).as_str(), &[]) {
        Ok(all_rows) => {
            for row in all_rows {
                let mut val: StockVal = StockVal::default();
                val.id = row.get(0);
                val.isin = row.get(1);
                val.time_epoch = row.get(2);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::cmd::create_stock::create_stock;
    
    #[test]
    fn test_cmd_get_stock_from_db() {
        /* create global state */
        let mut state: GlobalState = GlobalState::default();

        /* create stock to be tested */
        create_stock(&mut state, "haha").unwrap();

        /* insert some data into the stock */
        let mut client = db_connect(&mut state, DB_USER, DB_PASS).unwrap();
        client.execute("INSERT INTO asset_schema.haha VALUES (1, 999, 4, 50, 50, 10)", &[]).unwrap();

        /* test get_stock_from_db() */
        match get_stock_from_db(&mut state, "haha".into()) {
            Ok(vals) => {
                /* confirm that the data is correct */
                assert_eq!(vals.len(), 1);
                assert_eq!(vals[0].id, 1);
                assert_eq!(vals[0].isin, "999".to_string());
                assert_eq!(vals[0].time_epoch, 4);
                assert_eq!(vals[0].ask_price, 50);
                assert_eq!(vals[0].bid_price, 50);
                assert_eq!(vals[0].volume, 10);
            },
            Err(err) => panic!("TEST_CMD_GET_STOCK_FAILED: {}", err) 
        };
    }

    #[test]
    fn test_cmd_get_stock_from_db_since_epoch() {
        /* create global state */
        let mut state: GlobalState = GlobalState::default();
        
        /* create stock to be tested */
        create_stock(&mut state, "baba").unwrap();

        /* insert some data into the stock */
        let mut client = db_connect(&mut state, DB_USER, DB_PASS).unwrap();
        client.execute("INSERT INTO asset_schema.baba VALUES (1, 999, 4, 50, 50, 10)", &[]).unwrap();
        client.execute("INSERT INTO asset_schema.baba VALUES (2, 999, 5, 50, 50, 10)", &[]).unwrap();
        client.execute("INSERT INTO asset_schema.baba VALUES (3, 999, 6, 50, 50, 10)", &[]).unwrap();
        client.execute("INSERT INTO asset_schema.baba VALUES (4, 999, 7, 50, 50, 10)", &[]).unwrap();
        client.execute("INSERT INTO asset_schema.baba VALUES (5, 999, 8, 50, 50, 10)", &[]).unwrap();

        /* test get_stock_from_db_since_epoch() */
        match get_stock_from_db_since_epoch(&mut state, "baba".into(), 6) {
            Ok(vals) => {
                /* confirm that the data is correct */
                let mut counter = 6;
                for val in vals {
                    assert_eq!(val.id, counter-3);
                    assert_eq!(val.isin, "999".to_string());
                    assert_eq!(val.time_epoch, counter);
                    assert_eq!(val.ask_price, 50);
                    assert_eq!(val.bid_price, 50);
                    assert_eq!(val.volume, 10);
                    counter += 1;
                }
            },
            Err(err) => panic!("TEST_CMD_GET_STOCK_SINCE_EPOCH: {}", err)
        }
    }
}
