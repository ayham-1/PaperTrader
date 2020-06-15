#[derive(Default, PartialEq, Debug)]
pub struct Company {
    pub symbol: String,
    pub company_name: String,
    pub primary_exchange: String,
    pub sector: String,
    pub industry: String,
    pub primary_sic_code: i64,
    pub employees: i64,
}
