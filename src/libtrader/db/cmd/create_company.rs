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

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::initializer::db_init;

    #[test]
    fn test_cmd_create_company() {
        /* connect to db */
        let mut state: GlobalState = GlobalState::default();

        /* use create_company() */
        let mut company = Company::default();
        company.id = 1;
        company.symbol = "APP".to_string();
        company.isin = "131".to_string();
        company.company_name = "Apple1".to_string();
        company.primary_exchange = "NYSE".to_string();
        company.sector = "Tech".to_string();
        company.industry = "Tech".to_string();
        company.primary_sic_code = "1".to_string();
        company.employees = 1;
        
        match create_company(&mut state, company.clone()) {
            Ok(()) => {
                /* test create_company() */
                db_init(&mut state).unwrap();
                assert_eq!(state.companies["APP"].id, company.id);
                assert_eq!(state.companies["APP"].symbol, company.symbol);
                assert_eq!(state.companies["APP"].isin, company.isin);
                assert_eq!(state.companies["APP"].company_name, company.company_name);
                assert_eq!(state.companies["APP"].primary_exchange, company.primary_exchange);
                assert_eq!(state.companies["APP"].sector, company.sector);
                assert_eq!(state.companies["APP"].industry, company.industry);
                assert_eq!(state.companies["APP"].primary_sic_code, company.primary_sic_code);
                assert_eq!(state.companies["APP"].employees, company.employees);
            },
            Err(err) => panic!("TEST_CMD_CREATE_COMPANY_FAILED: {}", err),
        };
        
    }
}
