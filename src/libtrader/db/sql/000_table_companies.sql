CREATE TABLE companies (
    id               BIGSERIAL PRIMARY KEY
    symbol           text UNIQUE NOT NULL
    isin             text UNIQUE NOT NULL
    company_name     text NOT NULL
    primary_exchange text NOT NULL
    sector           text NOT NULL
    industry         text NOT NULL
    primary_sic_code text NOT NULL
    employees        bigint NOT NULL
)
