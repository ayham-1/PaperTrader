use crate::db::config::{DB_USER, DB_PASS};
use crate::db::initializer::db_connect;
use crate::ds::server::global_state::GlobalState;
use crate::ds::generic::company::Company;

/*
 * Creates a comany entry in database in public.companies.
 */
pub fn create_company(state: &mut GlobalState, company: Company) -> Result<(), String> {
    // Connect to database.
    let mut client = db_connect(state, DB_USER, DB_PASS)?;

    // Insert argument company into public.companies database table.
    match client.execute(
        "INSERT INTO public.companies VALUES ($1,$2, $3, $4, $5, $6, $7, $8, $9)",
        &[&company.id, &company.symbol, &company.isin, &company.company_name, 
            &company.primary_exchange, &company.sector, &company.industry,
            &company.primary_sic_code, &company.employees]) {
        Ok(_row) => {
            // add company to state
            state.companies.insert(company.symbol.to_string(), company);
            Ok(())
        },
        Err(error) => Err(format!("CMD_COMPANY_CREATE_FAILED: {}", error))
    }
}
