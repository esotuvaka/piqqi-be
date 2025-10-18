-- Migration number: 0001 	 2025-10-18T02:45:08.718Z
-- Create quotes table

DROP TABLE IF EXISTS quotes;

CREATE TABLE quotes (
    id                  TEXT PRIMARY KEY,            
    customer_id         TEXT NOT NULL,
    contact_id          TEXT NOT NULL,

    sender_company          TEXT NOT NULL,
    sender_address          TEXT NOT NULL,
    sender_city_state_zip   TEXT NOT NULL,

    client_company          TEXT NOT NULL,
    client_address          TEXT NOT NULL,
    client_city_state_zip   TEXT NOT NULL,
    client_country          TEXT NOT NULL,

    quote_name          TEXT NOT NULL,
    expires             TEXT NOT NULL,              -- stored as ISO timestamp string
    currency            TEXT NOT NULL,              -- 3-letter ISO code
    payment_terms       TEXT NOT NULL,
    delivery_terms      TEXT NOT NULL,
    status              TEXT NOT NULL,
    notes               TEXT NOT NULL,
    message             TEXT NOT NULL,

    tags                TEXT NOT NULL,              -- JSON-encoded array of strings
    version             INTEGER NOT NULL DEFAULT 1,

    created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
