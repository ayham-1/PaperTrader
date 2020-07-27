CREATE TABLE portfolios_schema.portfolios (
	id					BIGSERIAL PRIMARY KEY,
	userid				BIGSERIAL UNIQUE NOT NULL,
	position_history 	portfolio_pos[],
	open_positions		portfolio_pos[],
	is_active			BOOLEAN NOT NULL
)
