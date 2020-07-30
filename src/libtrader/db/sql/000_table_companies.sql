CREATE TABLE companies (
    id               BIGSERIAL PRIMARY KEY,
    symbol           TEXT UNIQUE NOT NULL,
    isin             TEXT UNIQUE NOT NULL,
    company_name     TEXT NOT NULL,
    primary_exchange TEXT NOT NULL,
    sector           TEXT NOT NULL,
    industry         TEXT NOT NULL,
    primary_sic_code TEXT NOT NULL,
    employees        BIGINT NOT NULL
)
