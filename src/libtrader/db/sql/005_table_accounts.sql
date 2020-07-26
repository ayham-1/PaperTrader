CREATE TABLE accounts_schema (
	id			BIGSERIAL PRIMARY KEY,
	username	TEXT UNIQUE NOT NULL,
	email		TEXT UNIQUE NOT NULL,
	is_pass		BOOLEAN NOT NULL,
	pass_hash	TEXT NOT NULL,
)
