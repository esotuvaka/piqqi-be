-- Migration number: 0001 	 2025-10-14T03:08:28.899Z
-- Purpose: Create quotes + polymorphic line_items tables
-- Timestamp: 2025-10-13T00:00:00Z

-- Enable foreign keys (required for D1 consistency)
PRAGMA foreign_keys = ON;

-- ===========================
-- QUOTES TABLE
-- ===========================
CREATE TABLE IF NOT EXISTS quotes (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_id     INTEGER NOT NULL,
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

CREATE INDEX IF NOT EXISTS idx_quotes_customer_id ON quotes(customer_id);
CREATE INDEX IF NOT EXISTS idx_quotes_status ON quotes(status);

-- ===========================
-- LINE ITEMS TABLE (Polymorphic)
-- ===========================
CREATE TABLE IF NOT EXISTS line_items (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_id     INTEGER NOT NULL,

    entity_type     TEXT NOT NULL CHECK (entity_type IN ('quote','salesorder','fulfillment','shipping')),
    entity_id       INTEGER NOT NULL,

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

    -- No strict FK, because entity_id may refer to multiple parent tables.
    -- You can still enforce referential integrity in application logic.

    created_at      INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_at      INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE INDEX IF NOT EXISTS idx_line_items_entity ON line_items(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_line_items_customer_id ON line_items(customer_id);
