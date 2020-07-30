CREATE TABLE sessions_schema.sessions (
	sess_id					TEXT NOT NULL,
	client_ip				TEXT NOT NULL,
	expiry_date				TIMESTAMP NOT NULL,
	is_active				BOOL NOT NULL
)
