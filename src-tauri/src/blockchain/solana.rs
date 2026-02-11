use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use super::types::{AccountBalance, ChainConfig, TokenBalance};

/// Query SOL balance and SPL token balances for a Solana address
pub async fn query_solana_balance(
    address: &str,
    config: &ChainConfig,
) -> Result<AccountBalance, String> {
    let client = RpcClient::new(config.rpc_url.clone());
    let pubkey = Pubkey::from_str(address)
        .map_err(|e| format!("Invalid Solana address: {}", e))?;

    // Get SOL balance
    let sol_balance = client
        .get_balance(&pubkey)
        .map_err(|e| format!("Failed to get SOL balance: {}", e))?;

    let sol_formatted = format!("{:.4}", sol_balance as f64 / 1_000_000_000.0);

    // SPL tokens - basic query
    let token_balances = Vec::new();
    // Note: Full SPL token enumeration requires token-account list RPC calls
    // which we'll add in a future iteration

    Ok(AccountBalance {
        address: address.to_string(),
        chain: "solana".to_string(),
        chain_name: "Solana".to_string(),
        chain_id: None,
        native_balance: TokenBalance {
            symbol: "SOL".to_string(),
            name: "Solana".to_string(),
            balance: sol_balance.to_string(),
            balance_formatted: sol_formatted,
            decimals: 9,
            contract_address: None,
            usd_value: None,
            is_native: true,
        },
        token_balances,
    })
}
