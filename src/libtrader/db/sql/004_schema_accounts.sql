CREATE SCHEMA accounts_schema;
CREATE ROLE accounts_schema_usr LOGIN PASSWORD 'PASSWORD';
GRANT CONNECT ON DATABASE pt_db TO accounts_schema_usr;
