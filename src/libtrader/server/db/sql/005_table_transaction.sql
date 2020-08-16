CREATE TABLE accounts_schema.transactions (
	id				BIGSERIAL PRIMARY KEY,
	user_id			BIGINT NOT NULL,
	stock_symbol 	TEXT NOT NULL,
	shares_size		BIGINT NOT NULL,
	shares_cost		BIGINT NOT NULL,
	is_buy			BOOLEAN NOT NULL
)
