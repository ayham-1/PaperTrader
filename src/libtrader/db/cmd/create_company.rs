use crate::db::init::db_connect;
use crate::ds::server::global_state::GlobalState;
use crate::ds::generic::company::Company;

pub fn create_company(state: &mut GlobalState, company: Company) -> Result<(), String> {
    // Connect to database.
    let mut client = db_connect(&state)?;

    // Insert argument company into public.companies database table.
    match client.execute(
        "INSERT INTO public.companies VALUES ($1,$2, $3, $4, $5, $6, $7, $8, $9)",
        &[&company.id, &company.symbol, &company.isin, &company.company_name, 
            &company.primary_exchange, &company.sector, &company.industry,
            &company.primary_sic_code, &company.employees]) {
        #[allow(unused_variables)] /* We do not need the number of rows modified. */
        Ok(row) => {
            // add company to state
            state.companies.insert(company.symbol.to_string(), company);
            Ok(())
        },
        Err(error) => Err(format!("Failed to create company with error: {}", error))
    }
}
