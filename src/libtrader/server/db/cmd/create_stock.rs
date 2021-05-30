use crate::common::misc::return_flags::ReturnFlags;

/// Creates a stock on the postgres SQL database.
///
/// Takes in a stock name and creates a table in the ```asset_schema``` schema
///
/// Arguments:
/// stock_name - The name of the stock to create.
///
/// Returns: nothing on success, a string containing the reason of failure on error.
///
/// Example:
/// ```rust
///    match create_stock("AAPL") {
///        Ok(()) => info!("created stock table"),
///        Err(err) => error!("failed to create stock table {}", err),
///    }
/// ```
pub async fn create_stock(
    sql_conn: &mut tokio_postgres::Client,
    stock_name: &str,
) -> Result<(), ReturnFlags> {
    /*
     * Creates a stock table in database in assets schema.
     */

    // Create the table.
    match sql_conn
        .execute(
            format!(
                "CREATE TABLE asset_schema.{} ( \
                        id                  BIGSERIAL PRIMARY KEY, \
                        isin                TEXT NOT NULL, \
                        time_epoch          BIGINT NOT NULL, \
                        ask_price           DOUBLE PRECISION NOT NULL, \
                        bid_price           DOUBLE PRECISION NOT NULL, \
                        volume              BIGINT NOT NULL \
                )",
                stock_name
            )
            .as_str(),
            &[],
        )
        .await
    {
        Ok(_rows) => Ok(()),
        Err(_) => Err(ReturnFlags::ServerDbCreateStockFailed),
    }
}
