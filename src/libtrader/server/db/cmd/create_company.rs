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
pub async fn create_company(
    sql_conn: &mut tokio_postgres::Client,
    company: Company,
) -> Result<Company, ReturnFlags> {
    /*
     * Creates a company entry in database in public.companies.
     */

    // Insert argument company into public.companies database table.
    match sql_conn
        .execute(
            "INSERT INTO public.companies VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[
                &company.id,
                &company.symbol,
                &company.isin,
                &company.company_name,
                &company.primary_exchange,
                &company.sector,
                &company.industry,
                &company.primary_sic_code,
                &company.employees,
            ],
        )
        .await
    {
        Ok(_row) => Ok(company),
        Err(_) => Err(ReturnFlags::ServerDbCreateCompanyFailed),
    }
}
