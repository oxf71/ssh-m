use alloy::primitives::{Address, U256, Bytes, keccak256};
use alloy::providers::ProviderBuilder;
use alloy::rpc::types::TransactionRequest;
use std::str::FromStr;

use super::types::{AccountBalance, ChainConfig, TokenBalance};

/// Query native + ERC20 token balances for an address on an EVM chain
pub async fn query_evm_balance(
    address: &str,
    config: &ChainConfig,
) -> Result<AccountBalance, String> {
    let rpc_url: reqwest::Url = config
        .rpc_url
        .parse()
        .map_err(|e| format!("Invalid RPC URL: {}", e))?;

    let provider = ProviderBuilder::new()
        .connect_http(rpc_url);

    let addr = Address::from_str(address)
        .map_err(|e| format!("Invalid address: {}", e))?;

    // Get native balance
    use alloy::providers::Provider;
    let native_balance: U256 = provider
        .get_balance(addr)
        .await
        .map_err(|e| format!("Failed to get native balance: {}", e))?;

    let native_formatted = format_balance(&native_balance, 18);

    // Query ERC20 tokens via raw eth_call
    let mut token_balances = Vec::new();
    for token in &config.tokens {
        let token_addr = Address::from_str(&token.contract_address).ok();
        let owner_addr = Address::from_str(address).ok();

        if let (Some(token_addr), Some(owner_addr)) = (token_addr, owner_addr) {
            // balanceOf(address) selector
            let selector = &keccak256("balanceOf(address)".as_bytes())[..4];
            let mut calldata = Vec::with_capacity(36);
            calldata.extend_from_slice(selector);
            calldata.extend_from_slice(&[0u8; 12]);
            calldata.extend_from_slice(owner_addr.as_slice());

            let tx = TransactionRequest::default()
                .to(token_addr)
                .input(Bytes::from(calldata).into());

            match provider.call(tx).await {
                Ok(result) => {
                    let balance = if result.len() >= 32 {
                        U256::from_be_slice(&result[..32])
                    } else {
                        U256::ZERO
                    };
                    let formatted = format_balance(&balance, token.decimals as u32);
                    token_balances.push(TokenBalance {
                        symbol: token.symbol.clone(),
                        name: token.name.clone(),
                        balance: balance.to_string(),
                        balance_formatted: formatted,
                        decimals: token.decimals,
                        contract_address: Some(token.contract_address.clone()),
                        usd_value: None,
                        is_native: false,
                    });
                }
                Err(e) => {
                    eprintln!("Failed to query token {}: {}", token.symbol, e);
                }
            }
        }
    }

    Ok(AccountBalance {
        address: address.to_string(),
        chain: "evm".to_string(),
        chain_name: config.name.clone(),
        chain_id: config.chain_id,
        native_balance: TokenBalance {
            symbol: config.symbol.clone(),
            name: config.name.clone(),
            balance: native_balance.to_string(),
            balance_formatted: native_formatted,
            decimals: 18,
            contract_address: None,
            usd_value: None,
            is_native: true,
        },
        token_balances,
    })
}

fn format_balance(balance: &U256, decimals: u32) -> String {
    if balance.is_zero() {
        return "0".to_string();
    }

    let divisor = U256::from(10u64).pow(U256::from(decimals));
    let whole = balance / divisor;
    let remainder = balance % divisor;

    if remainder.is_zero() {
        whole.to_string()
    } else {
        let remainder_str = format!("{:0>width$}", remainder, width = decimals as usize);
        let trimmed = remainder_str.trim_end_matches('0');
        let display_decimals = &trimmed[..trimmed.len().min(6)];
        format!("{}.{}", whole, display_decimals)
    }
}
