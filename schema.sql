-- schema.clickhouse.sql
CREATE TABLE IF NOT EXISTS raydium_swaps (
    tx_id String PRIMARY KEY,
    block_time DateTime,
    signer String,
    base_mint String,
    quote_mint String,
    base_amount UInt64,
    quote_amount UInt64,
    is_buy UInt8,
    sol_price String
) ENGINE = MergeTree()
ORDER BY (tx_id, block_time);

-- Required for substreams-sink-sql
CREATE TABLE IF NOT EXISTS cursors (
    id String,
    cursor String,
    block_num Int64,
    block_id String
) ENGINE = MergeTree
PRIMARY KEY (id);