CREATE ROLE accounts_schema_usr LOGIN PASSWORD 'PASSWORD';
CREATE SCHEMA accounts_schema AUTHORIZATION accounts_schema_usr;

GRANT CONNECT ON DATABASE pt_db TO accounts_schema_usr;

GRANT USAGE, CREATE ON SCHEMA accounts_schema TO accounts_schema_usr;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA accounts_schema TO accounts_schema_usr;
ALTER DEFAULT PRIVILEGES IN SCHEMA accounts_schema GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO accounts_schema_usr;

GRANT USAGE ON ALL SEQUENCES IN SCHEMA accounts_schema TO accounts_schema_usr;
ALTER DEFAULT PRIVILEGES IN SCHEMA accounts_schema GRANT USAGE ON SEQUENCES TO accounts_schema_usr;
