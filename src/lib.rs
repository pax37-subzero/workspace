mod constants;
mod pb;

use pb::mydata::v1::{RaydiumSwaps, TradeData};
use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use pb::sf::solana::dex::trades::v1::Output as DexOutput;

const MULTIPLIER: f64 = 1_000_000_000.0;

pub fn to_raw_amount(amount: f64) -> u64 {
    (amount * MULTIPLIER).round() as u64
}

#[substreams::handlers::map]
pub fn map_raydium_swaps(trades: DexOutput) -> Result<RaydiumSwaps, Error> {
    let raydium_trades: Vec<TradeData> = trades.data.into_iter()
        .filter(|trade| constants::is_supported_dex(&trade.outer_program, &trade.inner_program))
        .map(|trade| {
            let base_amount_raw = to_raw_amount(trade.base_amount);
            let quote_amount_raw = to_raw_amount(trade.quote_amount);
            
            TradeData {
                block_time: trade.block_time,
                tx_id: trade.tx_id,
                signer: trade.signer,
                base_mint: trade.base_mint,
                quote_mint: trade.quote_mint,
                base_amount: base_amount_raw,
                quote_amount: quote_amount_raw,
                txn_fee: trade.txn_fee,
                signer_sol_change: trade.signer_sol_change,
                ..Default::default() // для остальных полей используем значения по умолчанию
            }
        })
        .collect();
    
    Ok(RaydiumSwaps { data: raydium_trades })
}

#[substreams::handlers::map]
pub fn db_out(swaps: RaydiumSwaps) -> Result<DatabaseChanges, Error> {
    let mut database_changes = DatabaseChanges::default();
    
    for (idx, trade) in swaps.data.into_iter().enumerate() {
        database_changes
            .push_change("raydium_swaps", &trade.tx_id, idx as u64, Operation::Create)
            .change("block_time", (None, trade.block_time))
            .change("signer", (None, trade.signer))
            .change("base_mint", (None, trade.base_mint))
            .change("quote_mint", (None, trade.quote_mint))
            .change("base_amount", (None, trade.base_amount))
            .change("quote_amount", (None, trade.quote_amount))
            .change("txn_fee", (None, trade.txn_fee))
            .change("signer_sol_change", (None, trade.signer_sol_change));
    }
    
    Ok(database_changes)
}