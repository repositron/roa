-- Your SQL goes here
CREATE TABLE orders (
    id uuid PRIMARY KEY,
    table_id INTEGER NOT NULL,
    item VARCHAR NOT NULL,
    duration INTEGER DEFAULT 120 NOT NULL,
    expire_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_orders_table_id ON orders (table_id);