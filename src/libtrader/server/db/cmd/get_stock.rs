use crate::common::generic::stock_val::StockVal;
use crate::common::misc::return_flags::ReturnFlags;

/// Returns the whole stock data from the postgres SQL database.
///
/// Takes in a stock symbol and returns the whole data entries of the searched stock.
/// Should be used in Async contexts.
///
/// Arguments:
/// sql_conn - The SQL connection to use.
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
pub async fn get_stock_from_db(
    sql_conn: &mut tokio_postgres::Client,
    searched_symbol: &str,
) -> Result<Vec<StockVal>, ReturnFlags> {
    /*
     * Returns all stock values from database.
     */

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match sql_conn
        .query(
            format!("SELECT * FROM asset_schema.{}", searched_symbol).as_str(),
            &[],
        )
        .await
    {
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
/// Should be used in Async contexts.
///
/// Arguments:
/// sql_conn - The SQL connection to use.
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
pub async fn get_stock_from_db_since_epoch(
    sql_conn: &mut tokio_postgres::Client,
    searched_symbol: &str,
    time_epoch: i64,
) -> Result<Vec<StockVal>, ReturnFlags> {
    /*
     * Returns all stock values from database since a time epoch.
     */

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match sql_conn
        .query(
            format!(
                "SELECT * FROM asset_schema.{} WHERE time_epoch >= {}",
                searched_symbol, time_epoch
            )
            .as_str(),
            &[],
        )
        .await
    {
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
/// Should be used in Async contexts.
///
/// Arguments:
/// sql_conn - The SQL connection to use.
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
pub async fn get_stock_from_db_between_epochs(
    sql_conn: &mut tokio_postgres::Client,
    searched_symbol: &str,
    first_time_epoch: i64,
    second_time_epoch: i64,
) -> Result<Vec<StockVal>, ReturnFlags> {
    /*
     * Returns all stock values from database between two time epochs.
     */

    // Query database for table.
    let mut stocks: Vec<StockVal> = Vec::new();
    match sql_conn
        .query(
            format!(
                "SELECT * FROM asset_schema.{} WHERE time_epoch >= {} AND time_epoch <= {}",
                searched_symbol, first_time_epoch, second_time_epoch
            )
            .as_str(),
            &[],
        )
        .await
    {
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
