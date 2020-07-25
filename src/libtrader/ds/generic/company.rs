use postgres_types::{ToSql, FromSql};

#[derive(Default, PartialEq, Debug, ToSql, FromSql)]
pub struct Company {
    pub id: i8,
    pub symbol: String,
    pub isin: String,
    pub company_name: String,
    pub primary_exchange: String,
    pub sector: String,
    pub industry: String,
    pub primary_sic_code: String,
    pub employees: i8,
}
