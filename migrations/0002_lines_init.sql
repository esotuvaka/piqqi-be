-- Migration number: 0002 	 2025-10-18T02:48:33.179Z

DROP TABLE IF EXISTS line_items;

CREATE TABLE line_items (
    id              TEXT PRIMARY KEY,           
    customer_id     TEXT NOT NULL,

    entity_type     TEXT NOT NULL CHECK (entity_type IN ('quote','salesorder','fulfillment','shipping')),
    entity_id       TEXT NOT NULL,              -- references quotes.id or other entity IDs

    name            TEXT NOT NULL,
    sku             TEXT NOT NULL,
    quantity        REAL NOT NULL DEFAULT 1,
    unit_price      REAL NOT NULL DEFAULT 0,
    unit_cost       REAL NOT NULL DEFAULT 0,
    discount        REAL,
    discount_type   TEXT NOT NULL CHECK (discount_type IN ('percent','value')),
    notes           TEXT,
    message         TEXT,

    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ','now'))
);
