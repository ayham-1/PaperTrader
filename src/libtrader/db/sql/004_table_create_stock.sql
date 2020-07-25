CREATE TABLE asset_schema.stock_{} (
	id					BIGSERIAL PRIMARY KEY,
	isin				text NOT NULL,
	time_since_epoch 	timestamp NOT NULL,
	ask_price			bigint NOT NULL,
	bid_price			bigint NOT NULL,
	volume				bigint NOT NULL
)
