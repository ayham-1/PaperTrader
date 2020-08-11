CREATE TYPE transaction AS (
	stock_symbol 	TEXT,
	shares_size		BIGINT,
	shares_cost		BIGINT,
	is_buy			BOOLEAN
)
