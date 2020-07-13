CREATE TABLE asset_schema.stock_vals (
	id					BIGSERIAL PRIMARY KEY
	isin				text NOT NULL
	time_since_epoch 	timestamp NOT NULL
	ask_price			numeric NOT NULL
	bid_price			numeric NOT NULL
	volume				bigint NOT NULL
)
