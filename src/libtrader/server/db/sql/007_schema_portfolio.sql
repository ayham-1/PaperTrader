CREATE SCHEMA portfolios_schema;
CREATE ROLE portfolios_schema_usr LOGIN PASSWORD 'PASSWORD';
GRANT CONNECT ON DATABASE pt_db TO portfolios_schema_usr;
