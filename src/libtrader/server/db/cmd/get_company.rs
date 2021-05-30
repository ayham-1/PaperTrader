use crate::common::generic::company::Company;
use crate::common::misc::return_flags::ReturnFlags;

/// Returns a company from the postgres SQL database.
///
/// Takes in a company symbol and returns a company.
///
/// Arguments:
/// search_symbol - The specific company symbol to find.
///
/// Returns: a reference to the found company on success, and a string containing the reason of
/// failure on error.
///
/// Example:
/// ```rust
///    match get_company_from_db("AAPL".to_string()) {
///        Ok(found_company) => info!("we found it! {:?}", found_company),
///        Err(err) => error!("we must found the sacred company! err: {}", err),
///    }
/// ```
pub async fn get_company_from_db(
    sql_conn: &mut tokio_postgres::Client,
    searched_symbol: &str,
) -> Result<Company, ReturnFlags> {
    /*
     * Returns company entry from database
     */
    // Connect to database.
    match sql_conn
        .query(
            "SELECT * FROM public.companies WHERE symbol=$1",
            &[&searched_symbol],
        )
        .await
    {
        Ok(row) => {
            let mut found_company: Company = Company::default();
            found_company.id = row[0].get(0);
            found_company.symbol = row[0].get(1);
            found_company.isin = row[0].get(2);
            found_company.company_name = row[0].get(3);
            found_company.primary_exchange = row[0].get(4);
            found_company.sector = row[0].get(5);
            found_company.industry = row[0].get(6);
            found_company.primary_sic_code = row[0].get(7);
            found_company.employees = row[0].get(8);

            return Ok(found_company);
        }
        Err(_) => Err(ReturnFlags::ServerDbSearchCompanyNotFound),
    }
}
