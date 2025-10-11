-- Migration number: 0001 	 2025-10-11T03:59:31.469Z
-- quotes table
CREATE TABLE IF NOT EXISTS quotes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
	customer_id INTEGER NOT NULL,
    contact_id TEXT NOT NULL,
    sender_company TEXT NOT NULL,
    sender_address TEXT NOT NULL,
    sender_city_state_zip TEXT NOT NULL,
    client_company TEXT NOT NULL,
	client_address TEXT NOT NULL,
	client_city_state_zip TEXT NOT NULL,
    client_country TEXT NOT NULL,
    quote_name TEXT NOT NULL,
    expires INTEGER NOT NULL,
    currency TEXT NOT NULL,
    payment_terms TEXT NOT NULL,
    delivery_terms TEXT NOT NULL,
    status TEXT NOT NULL,
    notes TEXT NOT NULL,
    message TEXT NOT NULL,
    tags TEXT NOT NULL, -- store Vec<String> as JSON
    version INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

-- line_items table
CREATE TABLE IF NOT EXISTS line_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    quote_id INTEGER NOT NULL,
	customer_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    sku TEXT,
    quantity REAL NOT NULL DEFAULT 1,
    unit_price REAL NOT NULL DEFAULT 0,
    unit_cost REAL NOT NULL DEFAULT 0,
    profit REAL NOT NULL DEFAULT 0,
    margin REAL NOT NULL DEFAULT 0,
    discount REAL,
    discount_type TEXT CHECK(discount_type IN ('percent','value')),
    tax_rate REAL,
    notes TEXT,
    FOREIGN KEY (quote_id) REFERENCES quotes(id) ON DELETE CASCADE
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_line_items_quote_id ON line_items(quote_id);
