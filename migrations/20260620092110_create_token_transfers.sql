CREATE TABLE token_transfers (
    id SERIAL PRIMARY KEY,
    signature TEXT NOT NULL REFERENCES transactions(signature),
    wallet_address TEXT NOT NULL,
    mint TEXT NOT NULL,
    direction TEXT NOT NULL,
    amount_raw BIGINT NOT NULL,
    amount NUMERIC(38, 9),
    price_usd NUMERIC(38, 9),
    value_usd NUMERIC(38, 9)
);