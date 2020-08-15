CREATE TABLE portfolio_schema.portfolios (
	id					BIGSERIAL PRIMARY KEY,
	userid				BIGINT UNIQUE NOT NULL,
	open_positions		BIGINT[]
)
