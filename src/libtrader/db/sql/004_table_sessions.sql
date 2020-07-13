CREATE TABLE sessions_schema.sessions (
	sess_id					text NOT NULL
	client_ip				text NOT NULL
	expiry_date				timestamp NOT NULL
	is_active				bool NOT NULL
)
