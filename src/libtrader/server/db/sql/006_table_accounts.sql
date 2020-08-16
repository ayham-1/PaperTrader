CREATE TABLE accounts_schema.accounts (
	id				  BIGSERIAL PRIMARY KEY,
	username		  TEXT UNIQUE NOT NULL,

	email_hash		  TEXT UNIQUE NOT NULL,
	server_email_salt TEXT UNIQUE NOT NULL,
	client_email_salt TEXT UNIQUE NOT NULL,

	pass_hash		  TEXT UNIQUE NOT NULL,
	server_pass_salt  TEXT UNIQUE NOT NULL,
	client_pass_salt  TEXT UNIQUE NOT NULL
)
