use crate::db::config::{DB_USER, DB_PASS};
use crate::db::initializer::db_connect;
use crate::ds::generic::company::Company;
use crate::ds::generic::global_state::GlobalState;

/// Returns a company from the postgres SQL database.
///
/// Takes in a company symbol and returns a company.
///
/// Arguments:
/// state - The global state used.
/// search_symbol - The specific company symbol to find.
/// 
/// Returns: a reference to the found company on success, and a string containing the reason of
/// failure on error.
///
/// Example:
/// ```rust
///    match get_company_from_db(&mut state, "AAPL".to_string()) {
///        Ok(found_company) => info!("we found it! {:?}", found_company),
///        Err(err) => error!("we must found the sacred company! err: {}", err),
///    }
/// ```
pub fn get_company_from_db(state: &mut GlobalState, searched_symbol: String) -> Result<&Company, String> {
    /*
     * Returns company entry from database
     */
    // Connect to database.
    let mut client = db_connect(state, DB_USER, DB_PASS)?;
    match client.query("SELECT * FROM public.companies WHERE symbol=$1",
                       &[&searched_symbol]) {
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

            // add found_company to state.
            state.companies.insert(row[0].get(1), found_company);

            match state.companies.get(&searched_symbol) {
                Some(company) => Ok(company),
                None => Err("DB_SEARCH_COMPANY_NOT_FOUND".into())
            }
        },
        Err(err) => Err(format!("DB_SEARCH_COMPANY_NOT_FOUND: {}", err)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::cmd::create_company::create_company;

    #[test]
    fn test_cmd_get_company_from_db() {
        /* create global state */
        let mut state: GlobalState = GlobalState::default();

        /* create a new company */
        let mut company = Company::default();
        company.id = 454;
        company.symbol = "BBP".to_string();
        company.isin = "14141".to_string();
        company.company_name = "BBP?".to_string();
        company.primary_exchange = "NYSE".to_string();
        company.sector = "Tech".to_string();
        company.industry = "Tech".to_string();
        company.primary_sic_code = "141499".to_string();
        company.employees = 1;
        create_company(&mut state, company.clone()).unwrap();

        /* test get_company_from_db() */
        match get_company_from_db(&mut state, "BBP".into()) {
            Ok(found_company) => {
                assert_eq!(found_company.id, company.id);
                assert_eq!(found_company.symbol, company.symbol);
                assert_eq!(found_company.isin, company.isin);
                assert_eq!(found_company.company_name, company.company_name);
                assert_eq!(found_company.primary_exchange, company.primary_exchange);
                assert_eq!(found_company.sector, company.sector);
                assert_eq!(found_company.industry, company.industry);
                assert_eq!(found_company.primary_sic_code, company.primary_sic_code);
                assert_eq!(found_company.employees, company.employees);
            },
            Err(err) => panic!("TEST_CMD_GET_COMPANY_FROM_DB_FAILED: {}", err)
        }
    }
}
