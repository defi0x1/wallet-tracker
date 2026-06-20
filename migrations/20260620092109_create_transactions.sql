CREATE TABLE transactions (
    signature TEXT PRIMARY KEY,
    wallet_address TEXT NOT NULL REFERENCES wallets(address),
    slot BIGINT NOT NULL,
    block_time TIMESTAMPTZ NOT NULL,
    tx_type TEXT,
    status TEXT NOT NULL,
    fee_lamports BIGINT,
    raw_meta JSONB
);