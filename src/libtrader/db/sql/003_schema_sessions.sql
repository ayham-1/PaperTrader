CREATE SCHEMA sessions_schema;
CREATE ROLE sessions_schema_usr LOGIN PASSWORD 'PASSWORD';
GRANT CONNECT ON DATABASE pt_db TO sessions_schema_usr;
