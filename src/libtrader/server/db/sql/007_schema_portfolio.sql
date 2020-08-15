CREATE ROLE portfolio_schema_usr LOGIN PASSWORD 'PASSWORD';
CREATE SCHEMA portfolio_schema AUTHORIZATION portfolio_schema_usr;

GRANT CONNECT ON DATABASE pt_db TO portfolio_schema_usr;

GRANT USAGE, CREATE ON SCHEMA portfolio_schema TO portfolio_schema_usr;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA portfolio_schema TO portfolio_schema_usr;
ALTER DEFAULT PRIVILEGES IN SCHEMA portfolio_schema GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO portfolio_schema_usr;

GRANT USAGE ON ALL SEQUENCES IN SCHEMA portfolio_schema TO portfolio_schema_usr;
ALTER DEFAULT PRIVILEGES IN SCHEMA portfolio_schema GRANT USAGE ON SEQUENCES TO portfolio_schema_usr;
