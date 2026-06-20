CREATE TABLE telegram_users (
    telegram_id BIGINT PRIMARY KEY,
    username TEXT,
    chat_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);