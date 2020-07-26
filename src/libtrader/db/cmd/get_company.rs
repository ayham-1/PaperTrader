use crate::db::init::db_connect;
use crate::ds::server::global_state::GlobalState;
use crate::ds::generic::company::Company;

/*
 * Returns company entry from database
 */
pub fn get_company_from_db(state: &mut GlobalState, searched_symbol: String) -> Result<&Company, String> {
    /*
     * Searches in database if company is available
     */
    // Connect to database.
    let mut client = db_connect(&state)?;
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
                None => Err("DB_SEARCH_COMPANY_NOT_FOUND".to_string())
            }
        },
        Err(err) => Err(format!("DB_SEARCH_COMPANY_NOT_FOUND: {}", err)),
    }
}
