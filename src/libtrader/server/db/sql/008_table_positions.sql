CREATE TABLE portfolio_schema.positions (
	id 					BIGSERIAL PRIMARY KEY,
	stock_symbol		TEXT NOT NULL,
	stock_open_amount	BIGINT NOT NULL,
	stock_open_price	DOUBLE PRECISION NOT NULL,
	stock_open_cost		DOUBLE PRECISION NOT NULL,
	stock_close_amount	BIGINT NOT NULL,
	stock_close_price	DOUBLE PRECISION NOT NULL,
	open_epoch			BIGINT NOT NULL,
	close_epoch			BIGINT NOT NULL,
	is_buy				BOOLEAN NOT NULL,
	is_open				BOOLEAN NOT NULL
)
