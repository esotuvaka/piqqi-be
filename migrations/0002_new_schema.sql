-- Migration number: 0002 	 2025-10-18T01:51:03.968Z
-- Purpose: Create quotes + polymorphic line_items tables with TEXT IDs
-- Timestamp: 2025-10-17T00:00:00Z

-- Drop existing tables
DROP TABLE IF EXISTS line_items;
DROP TABLE IF EXISTS quotes;

-- Enable foreign keys (required for D1 consistency)
PRAGMA foreign_keys = ON;

-- ===========================
-- QUOTES TABLE
-- ===========================
CREATE TABLE quotes (
    id              TEXT PRIMARY KEY, -- Use UUID or similar
    customer_id     TEXT NOT NULL,
    contact_id      TEXT NOT NULL,

    sender_company          TEXT NOT NULL,
    sender_address          TEXT NOT NULL,
    sender_city_state_zip   TEXT NOT NULL,

    client_company          TEXT NOT NULL,
    client_address          TEXT NOT NULL,
    client_city_state_zip   TEXT NOT NULL,
    client_country          TEXT NOT NULL,

    quote_name      TEXT NOT NULL,
    expires         INTEGER NOT NULL, -- epoch seconds
    currency        TEXT NOT NULL,
    payment_terms   TEXT NOT NULL,
    delivery_terms  TEXT NOT NULL,
    status          TEXT NOT NULL,
    notes           TEXT NOT NULL,
    message         TEXT NOT NULL,
    tags            TEXT NOT NULL, -- serialized JSON array
    version         INTEGER NOT NULL DEFAULT 1,

    created_at      INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_at      INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE INDEX idx_quotes_customer_id ON quotes(customer_id);
CREATE INDEX idx_quotes_status ON quotes(status);

-- ===========================
-- LINE ITEMS TABLE (Polymorphic)
-- ===========================
CREATE TABLE line_items (
    id              TEXT PRIMARY KEY, -- Use UUID or similar
    customer_id     TEXT NOT NULL,

    entity_type     TEXT NOT NULL CHECK (entity_type IN ('quote','salesorder','fulfillment','shipping')),
    entity_id       TEXT NOT NULL, -- references quotes.id or other entity IDs as TEXT

    name            TEXT NOT NULL,
    sku             TEXT NOT NULL,
    quantity        INTEGER NOT NULL DEFAULT 1,
    unit_price      REAL NOT NULL DEFAULT 0,
    unit_cost       REAL NOT NULL DEFAULT 0,
    profit          REAL NOT NULL DEFAULT 0,
    margin          REAL NOT NULL DEFAULT 0,
    discount        REAL,
    discount_type   TEXT NOT NULL CHECK (discount_type IN ('percent','value')),
    tax_rate        REAL,
    notes           TEXT,

    created_at      INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_at      INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE INDEX idx_line_items_entity ON line_items(entity_type, entity_id);
CREATE INDEX idx_line_items_customer_id ON line_items(customer_id);
