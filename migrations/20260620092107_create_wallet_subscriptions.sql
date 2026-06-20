CREATE TABLE wallet_subscriptions (
    id SERIAL PRIMARY KEY,
    telegram_id BIGINT NOT NULL REFERENCES telegram_users(telegram_id),
    wallet_address TEXT NOT NULL REFERENCES wallets(address),
    notify_swaps BOOLEAN NOT NULL DEFAULT true,
    notify_transfers BOOLEAN NOT NULL DEFAULT true,
    min_value_usd NUMERIC NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (telegram_id, wallet_address)
);