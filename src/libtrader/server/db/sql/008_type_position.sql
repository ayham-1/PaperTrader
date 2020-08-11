CREATE TYPE portfolio_pos AS (
	is_buy				BOOLEAN,
	stock_symbol		TEXT,
	stock_open_amount	BIGINT,
	stock_open_price	FLOAT8,
	stock_open_cost		FLOAT8,
	stock_close_amount	BIGINT,
	stock_close_price	FLOAT8,
	open_epoch			BIGINT,
	close_epoch			BIGINT,
	is_open				BOOLEAN
)
