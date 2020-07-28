use crate::db::config::{*};
use crate::ds::server::global_state::GlobalState;
use crate::ds::generic::company::Company;

pub fn db_gen_connect_str(state: &mut GlobalState, user: &'static str, pass: &'static str) {
    state.db_connect_str = format!("host={} port={} dbname={} user={} password={}",
                                   DB_HOST, DB_HOST_PORT, DB_NAME, user, pass);
}

pub fn db_connect(state: &GlobalState) -> Result<postgres::Client, String> {
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
    let mut client = db_connect(&state)?;

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
