use crate::server::db::config::{DB_USER, DB_PASS};
use crate::server::db::initializer::db_connect;
use crate::common::generic::company::Company;
use crate::common::misc::return_flags::ReturnFlags;

/// Creates a company on the postgres SQL database.
/// 
/// Takes in a company and writes an entry in public.companies.
///
/// Arguments:
/// company - The company to create.
///
/// Returns: the company, a string containing reason of failure on error.
///
/// Example:
/// ```rust
///    match create_company(company) {
///        Ok(()) => info!("created company"),
///        Err(err) => error!("Failed to create company with error: {}", err),
///    }
/// ```
pub fn create_company(company: Company) -> Result<Company, ReturnFlags> {
    /*
     * Creates a company entry in database in public.companies.
     */
    // Connect to database.
    let mut client = db_connect(DB_USER, DB_PASS)?;

    // Insert argument company into public.companies database table.
    match client.execute(
        "INSERT INTO public.companies VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        &[&company.id, &company.symbol, &company.isin, &company.company_name, 
            &company.primary_exchange, &company.sector, &company.industry,
            &company.primary_sic_code, &company.employees]) {
        Ok(_row) => Ok(company),
        Err(_) => Err(ReturnFlags::SERVER_DB_CREATE_COMPANY_FAILED)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_create_company() {
        /* create a new company */
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
        
        /* test create_company() with created company */
        match create_company(company.clone()) {
            Ok(company) => {
                /* test create_company() */
                assert_eq!(company.id, company.id);
                assert_eq!(company.symbol, company.symbol);
                assert_eq!(company.isin, company.isin);
                assert_eq!(company.company_name, company.company_name);
                assert_eq!(company.primary_exchange, company.primary_exchange);
                assert_eq!(company.sector, company.sector);
                assert_eq!(company.industry, company.industry);
                assert_eq!(company.primary_sic_code, company.primary_sic_code);
                assert_eq!(company.employees, company.employees);
            },
            Err(err) => panic!("TEST_CMD_CREATE_COMPANY_FAILED: {}", err),
        };
    }
}
