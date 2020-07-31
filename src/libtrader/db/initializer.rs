use crate::db::config::{DB_HOST, DB_HOST_PORT, DB_USER, DB_NAME, DB_PASS};
use crate::ds::server::global_state::GlobalState;
use crate::ds::generic::company::Company;

pub fn db_connect(state: &mut GlobalState, user: &'static str, 
                  pass: &'static str) -> Result<postgres::Client, String> {
    /* Generate the requested string */
    state.db_connect_str = format!("host={} port={} dbname={} user={} password={}",
                                   DB_HOST, DB_HOST_PORT, DB_NAME, user, pass);
    match postgres::Client::connect(state.db_connect_str.as_str(), postgres::NoTls) {
        Ok(client) => return Ok(client),
        Err(error) => return Err(format!("DB_FAILED_INIT: {}", error))
    }
}

pub fn db_init(state: &mut GlobalState) -> Result<(), String> {
    /* 
     * Load companies from database
     */
    // Connect to database
    let mut client = db_connect(state, DB_USER, DB_PASS)?;

    // Query database for companies and store them.
    for row in client.query("SELECT * FROM public.companies", &[]).unwrap() {
        let mut company: Company = Company::default();
        company.id = row.get(0);
        company.symbol = row.get(1);
        company.isin = row.get(2);
        company.company_name = row.get(3);
        company.primary_exchange = row.get(4);
        company.sector = row.get(5);
        company.industry = row.get(6);
        company.primary_sic_code = row.get(7);
        company.employees = row.get(8);

        // add company to state.
        state.companies.insert(company.symbol.to_string(), company);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_db_connect() {
        let mut state: GlobalState = GlobalState::default();
        match db_connect(&mut state, DB_USER, DB_PASS) {
            Ok(client) => assert_eq!(client.is_closed(), false),
            Err(err) => panic!("TEST_DB_CONNECT_FAILED: {}", err),
        }
    }
    #[test]
    fn test_db_init() {
        /* connect to db */
        let mut state: GlobalState = GlobalState::default();
        let mut client = db_connect(&mut state, DB_USER, DB_PASS).unwrap();

        /* add test compnay */
        let mut company = Company::default();
        company.id = 123;
        company.symbol = "AAPL".to_string();
        company.isin = "1".to_string();
        company.company_name = "Apple".to_string();
        company.primary_exchange = "NYSE".to_string();
        company.sector = "Tech".to_string();
        company.industry = "Tech".to_string();
        company.primary_sic_code = "1".to_string();
        company.employees = 1;
        client.execute(
            "INSERT INTO public.companies VALUES ($1,$2, $3, $4, $5, $6, $7, $8, $9)",
            &[&company.id, &company.symbol, &company.isin, &company.company_name, 
            &company.primary_exchange, &company.sector, &company.industry,
            &company.primary_sic_code, &company.employees]).unwrap();
        /* test db init */ 
        match db_init(&mut state) {
            Ok(()) => {
                println!("{}", state);
                assert_eq!(state.companies["AAPL"].id, company.id);
                assert_eq!(state.companies["AAPL"].symbol, company.symbol);
                assert_eq!(state.companies["AAPL"].isin, company.isin);
                assert_eq!(state.companies["AAPL"].company_name, company.company_name);
                assert_eq!(state.companies["AAPL"].primary_exchange, company.primary_exchange);
                assert_eq!(state.companies["AAPL"].sector, company.sector);
                assert_eq!(state.companies["AAPL"].industry, company.industry);
                assert_eq!(state.companies["AAPL"].primary_sic_code, company.primary_sic_code);
                assert_eq!(state.companies["AAPL"].employees, company.employees);
            },
            Err(err) => panic!("TEST_DB_INIT_FAILED: {}", err)
        };
    }

}
