#[derive(Default, PartialEq, Debug)]
pub struct Company {
    pub id: i64,
    pub symbol: String,
    pub isin: String,
    pub company_name: String,
    pub primary_exchange: String,
    pub sector: String,
    pub industry: String,
    pub primary_sic_code: String,
    pub employees: i64,
}
impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {}, {}, {}, {})", self.id, self.symbol, self.isin, self.company_name, self.primary_exchange, self.sector, self.industry, self.primary_sic_code, self.employees)
    }
}
