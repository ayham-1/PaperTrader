CREATE TYPE portfolio_pos AS (
	stock_symbol		TEXT,
	stock_open_amount	BIGINT,
	stock_open_price	MONEY,
	stock_open_cost		MONEY,
	stock_close_amount	MONEY,
	stock_close_price	MONEY,
	open_epoch			BIGINT,
	close_epoch			BIGINT,
	is_buy				BOOLEAN,
	is_open				BOOLEAN
)
