mod constants;
mod pb;

use pb::mydata::v1::{RaydiumSwaps, TradeData};
use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use pb::sf::solana::dex::trades::v1::Output as DexOutput;
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const MULTIPLIER: f64 = 1_000_000_000.0;

#[derive(Debug)]
enum SwapType {
    Buy,
    Sell,
}

pub fn to_raw_amount(amount: f64) -> u64 {
    (amount.abs() * MULTIPLIER).round() as u64
}

fn analyze_swap(base_mint: &str, quote_mint: &str, base_amount: u64, quote_amount: u64) -> Option<(SwapType, f64)> {
    let (swap_type, sol_amount, token_amount) = if base_mint == SOL_MINT {
        (SwapType::Buy, base_amount, quote_amount)
    } else if quote_mint == SOL_MINT {
        (SwapType::Sell, quote_amount, base_amount)
    } else {
        return None;
    };

    let sol_price = if token_amount > 0 {
        sol_amount as f64 / token_amount as f64
    } else {
        0.0
    };

    Some((swap_type, sol_price))
}

#[substreams::handlers::map]
pub fn map_raydium_swaps(trades: DexOutput) -> Result<RaydiumSwaps, Error> {
    let raydium_trades: Vec<TradeData> = trades.data.into_iter()
        .filter_map(|trade| {
            let base_amount_raw = to_raw_amount(trade.base_amount);
            let quote_amount_raw = to_raw_amount(trade.quote_amount);
            
            analyze_swap(
                &trade.base_mint,
                &trade.quote_mint,
                base_amount_raw,
                quote_amount_raw
            ).map(|(swap_type, sol_price)| {
                TradeData {
                    block_time: trade.block_time,
                    tx_id: trade.tx_id,
                    signer: trade.signer,
                    base_mint: trade.base_mint,
                    quote_mint: trade.quote_mint,
                    base_amount: base_amount_raw,
                    quote_amount: quote_amount_raw,
                    is_buy: matches!(swap_type, SwapType::Buy) as i32,
                    sol_price: sol_price.to_string(),
                }
            })
        })
        .collect();
    
    Ok(RaydiumSwaps { data: raydium_trades })
}

#[substreams::handlers::map]
pub fn db_out(trades: DexOutput) -> Result<DatabaseChanges, Error> {
    let mut database_changes = DatabaseChanges::default();
    
    for (idx, trade) in trades.data.into_iter().enumerate() {
        let base_amount_raw = to_raw_amount(trade.base_amount);
        let quote_amount_raw = to_raw_amount(trade.quote_amount);
        
        if let Some((swap_type, sol_price)) = analyze_swap(
            &trade.base_mint,
            &trade.quote_mint,
            base_amount_raw,
            quote_amount_raw
        ) {
            let unique_id = format!("{}:{}", trade.tx_id, idx);
            
            database_changes
                .push_change("raydium_swaps", &unique_id, idx as u64, Operation::Create)
                .change("block_time", (None, trade.block_time))
                .change("signer", (None, trade.signer))
                .change("base_mint", (None, trade.base_mint))
                .change("quote_mint", (None, trade.quote_mint))
                .change("base_amount", (None, base_amount_raw))
                .change("quote_amount", (None, quote_amount_raw))
                .change("is_buy", (None, matches!(swap_type, SwapType::Buy)))
                .change("sol_price", (None, sol_price.to_string()));
        }
    }
    
    Ok(database_changes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_raw_amount() {
        assert_eq!(to_raw_amount(15_000_000.0), 15_000_000_000_000_000);
        assert_eq!(to_raw_amount(-378_555.325607), 378_555_325_607_000);
        assert_eq!(to_raw_amount(0.486662234), 486_662_234);
        assert_eq!(to_raw_amount(-0.486662234), 486_662_234);
    }

    #[test]
    fn test_analyze_swap() {
        // Тест покупки токена за SOL
        let (swap_type, price) = analyze_swap(
            SOL_MINT,
            "other_token",
            1_000_000_000,
            100_000_000
        ).unwrap();
        assert!(matches!(swap_type, SwapType::Buy));
        assert_eq!(price, 0.01);

        // Тест продажи токена за SOL
        let (swap_type, price) = analyze_swap(
            "other_token",
            SOL_MINT,
            100_000_000,
            1_000_000_000
        ).unwrap();
        assert!(matches!(swap_type, SwapType::Sell));
        assert_eq!(price, 0.01);

        // Тест свопа без SOL
        assert!(analyze_swap(
            "token1",
            "token2",
            1_000_000,
            1_000_000
        ).is_none());
    }
}