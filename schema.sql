-- schema.clickhouse.sql
CREATE TABLE IF NOT EXISTS raydium_swaps (
    id String PRIMARY KEY, -- key: tx_id 
    block_time DateTime,
    signer String,
    base_mint String,
    quote_mint String,
    base_amount UInt64,
    quote_amount UInt64,
    txn_fee UInt64,
    signer_sol_change Int64
) ENGINE = MergeTree
ORDER BY (id, block_time);

-- Required for substreams-sink-sql
CREATE TABLE IF NOT EXISTS cursors (
    id String,
    cursor String,
    block_num Int64,
    block_id String
) ENGINE = MergeTree
PRIMARY KEY (id);