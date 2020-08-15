CREATE TABLE portfolio_schema.portfolios (
	id					BIGSERIAL PRIMARY KEY,
	userid				BIGSERIAL UNIQUE NOT NULL,
	open_positions		portfolio_pos[],
	is_active			BOOLEAN NOT NULL
)
