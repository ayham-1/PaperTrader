use crate::common::generic::stock_val::StockVal;
use crate::common::misc::return_flags::ReturnFlags;
use crate::server::db::config::{DB_PASS, DB_USER};
use crate::server::db::initializer::db_connect;

/// Returns the whole stock data from the postgres SQL database.
///
/// Takes in a stock symbol and returns the whole data entries of the searched stock.
///
/// Arguments:
/// searched_symbol - The name of the stock table.
///
/// Returns: a Vec<StockVal> on success, and a string containing the reason of failure on error.
///
/// Example:
/// ```rust
///    match get_stock_from_db("AAPL".into()) {
///         Ok(vals) => {
///             /* do something with the values */
///         },
///         Err(err) => panic!("failed to get the stock value, reason: {}", err)
///   };
/// ```
pub fn get_stock_from_db(searched_symbol: &str) -> Result<Vec<StockVal>, ReturnFlags> {
    /*
     * Returns all stock values from database.
     */
    // Connect to database.
    let mut client = db_connect(DB_USER, DB_PASS)?;

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match client.query(
        format!("SELECT * FROM asset_schema.{}", searched_symbol).as_str(),
        &[],
    ) {
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
        }
        Err(_) => Err(ReturnFlags::ServerDbSearchStockNotFound),
    }
}

/// Returns stock data since an unix epoch from the postgres SQL database.
///
/// Takes in a stock symbol and returns the data entries after a specified epoch of the searched stock.
///
/// Arguments:
/// searched_symbol - The name of the stock table.
/// time_epoch - The time from which the stock data retrieved.
///
/// Returns: a Vec<StockVal> on success, and a string containing the reason of failure on error.
///
/// Example:
/// ```rust
///     match get_stock_from_db_since_epoch("AAPL".into(), 123456) {
///         Ok(vals) => {
///             /* do something with the filtered values */
///         },
///         Err(err) => panic!("failed to get the stock value, reason: {}", err)
///     };
/// ```
pub fn get_stock_from_db_since_epoch(
    searched_symbol: &str,
    time_epoch: i64,
) -> Result<Vec<StockVal>, ReturnFlags> {
    /*
     * Returns all stock values from database since a time epoch.
     */
    // Connect to database.
    let mut client = db_connect(DB_USER, DB_PASS)?;

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match client.query(
        format!(
            "SELECT * FROM asset_schema.{} WHERE time_epoch >= {}",
            searched_symbol, time_epoch
        )
        .as_str(),
        &[],
    ) {
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
        }
        Err(_) => Err(ReturnFlags::ServerDbSearchStockNotFound),
    }
}

/// Returns stock data between two unix epochs from the postgres SQL database.
///
/// Takes in a stock symbol and returns the data entries between two specified unix epochs of the searched
/// stock.
///
/// Arguments:
/// searched_symbol - The name of the stock table.
/// first_time_epoch - The time from which the stock data is first retrieved.
/// second_time_epoch - The time from which the stock data ends.
///
/// Returns: a Vec<StockVal> on success, and a string containing the reason of failure on error.
///
/// Example:
/// ```rust
///    match get_stock_from_db_between_epochs("AAPL".into(), 123456, 123459) {
///         Ok(vals) => {
///             /* do something with the filtered values */
///         },
///         Err(err) => panic!("failed to get the stock value, reason: {}", err)
///   };
/// ```
pub fn get_stock_from_db_between_epochs(
    searched_symbol: &str,
    first_time_epoch: i64,
    second_time_epoch: i64,
) -> Result<Vec<StockVal>, ReturnFlags> {
    /*
     * Returns all stock values from database between two time epochs.
     */
    // Connect to database.
    let mut client = db_connect(DB_USER, DB_PASS)?;

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match client.query(
        format!(
            "SELECT * FROM asset_schema.{} WHERE time_epoch >= {} AND time_epoch <= {}",
            searched_symbol, first_time_epoch, second_time_epoch
        )
        .as_str(),
        &[],
    ) {
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
        }
        Err(_) => Err(ReturnFlags::ServerDbSearchStockNotFound),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::server::db::cmd::create_stock::create_stock;

    #[test]
    fn test_cmd_get_stock_from_db() {
        /* create stock to be tested */
        create_stock("haha").unwrap();

        /* insert some data into the stock */
        let mut client = db_connect(DB_USER, DB_PASS).unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.haha VALUES (1, 999, 4, 50.0, 50.0, 10)",
                &[],
            )
            .unwrap();

        /* test get_stock_from_db() */
        match get_stock_from_db("haha".into()) {
            Ok(vals) => {
                /* confirm that the data is correct */
                assert_eq!(vals.len(), 1);
                assert_eq!(vals[0].id, 1);
                assert_eq!(vals[0].isin, "999".to_string());
                assert_eq!(vals[0].time_epoch, 4);
                assert_eq!(vals[0].ask_price, 50.0);
                assert_eq!(vals[0].bid_price, 50.0);
                assert_eq!(vals[0].volume, 10);
            }
            Err(err) => panic!("TEST_CMD_GET_STOCK_FAILED: {}", err),
        };
    }

    #[test]
    fn test_cmd_get_stock_from_db_since_epoch() {
        /* create stock to be tested */
        create_stock("baba").unwrap();

        /* insert some data into the stock */
        let mut client = db_connect(DB_USER, DB_PASS).unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.baba VALUES (1, 999, 4, 50, 50, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.baba VALUES (2, 999, 5, 50, 50, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.baba VALUES (3, 999, 6, 50, 50, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.baba VALUES (4, 999, 7, 50, 50, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.baba VALUES (5, 999, 8, 50, 50, 10)",
                &[],
            )
            .unwrap();

        /* test get_stock_from_db_since_epoch() */
        match get_stock_from_db_since_epoch("baba".into(), 6) {
            Ok(vals) => {
                /* confirm that the data is correct */
                let mut counter = 6;
                for val in vals {
                    assert_eq!(val.id, counter - 3);
                    assert_eq!(val.isin, "999".to_string());
                    assert_eq!(val.time_epoch, counter);
                    assert_eq!(val.ask_price, 50.0);
                    assert_eq!(val.bid_price, 50.0);
                    assert_eq!(val.volume, 10);
                    counter += 1;
                }
            }
            Err(err) => panic!("TEST_CMD_GET_STOCK_SINCE_EPOCH_FAILED: {}", err),
        }
    }

    #[test]
    fn test_cmd_get_stock_from_db_between_epochs() {
        /* create stock to be tested */
        create_stock("vava").unwrap();

        /* insert some data into the stock */
        let mut client = db_connect(DB_USER, DB_PASS).unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.vava VALUES (1, 999, 4, 50.0, 50.0, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.vava VALUES (2, 999, 5, 50.0, 50.0, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.vava VALUES (3, 999, 6, 50.0, 50.0, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.vava VALUES (4, 999, 7, 50.0, 50.0, 10)",
                &[],
            )
            .unwrap();
        client
            .execute(
                "INSERT INTO asset_schema.vava VALUES (5, 999, 8, 50.0, 50.0, 10)",
                &[],
            )
            .unwrap();

        /* test get_stock_from_db_between_epochs() */
        match get_stock_from_db_between_epochs("vava".into(), 5, 7) {
            Ok(vals) => {
                /* confirm that the data is correct */
                assert_eq!(vals[0].id, 2);
                assert_eq!(vals[0].isin, "999".to_string());
                assert_eq!(vals[0].time_epoch, 5);
                assert_eq!(vals[0].ask_price, 50.0);
                assert_eq!(vals[0].bid_price, 50.0);
                assert_eq!(vals[0].volume, 10);

                assert_eq!(vals[1].id, 3);
                assert_eq!(vals[1].isin, "999".to_string());
                assert_eq!(vals[1].time_epoch, 6);
                assert_eq!(vals[1].ask_price, 50.0);
                assert_eq!(vals[1].bid_price, 50.0);
                assert_eq!(vals[1].volume, 10);

                assert_eq!(vals[2].id, 4);
                assert_eq!(vals[2].isin, "999".to_string());
                assert_eq!(vals[2].time_epoch, 7);
                assert_eq!(vals[2].ask_price, 50.0);
                assert_eq!(vals[2].bid_price, 50.0);
                assert_eq!(vals[2].volume, 10);
            }
            Err(err) => panic!("TEST_CMD_GET_STOCK_BETWEEN_EPOCHS_FAILED: {}", err),
        }
    }
}
