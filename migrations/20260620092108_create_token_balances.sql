CREATE TABLE token_balances (
    id SERIAL PRIMARY KEY,
    wallet_address TEXT NOT NULL REFERENCES wallets(address),
    mint TEXT NOT NULL,
    amount_raw BIGINT NOT NULL,
    amount NUMERIC(38, 9) NOT NULL,
    price_usd NUMERIC(38, 9),
    value_usd NUMERIC(38, 9),
    slot BIGINT,
    snapshot_at TIMESTAMPTZ NOT NULL,
    UNIQUE (wallet_address, mint, snapshot_at)
);